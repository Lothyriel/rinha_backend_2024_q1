use async_sqlite::rusqlite::{Connection, OptionalExtension};

use crate::{
    db::{get_extract_data, insert_transaction_data},
    models::*,
    routes::ErrorResponse,
};

pub fn add_transaction(
    conn: &mut Connection,
    client_id: u32,
    request: TransactionRequest,
) -> Result<TransactionResponse, ErrorResponse> {
    if request.description.len() > 10 || request.description.is_empty() {
        return Err(ErrorResponse::InvalidDescription);
    }

    let client = get_client(conn, client_id)?;

    let new_balance = get_new_balance(&request, &client).ok_or(ErrorResponse::NotEnoughLimit)?;

    insert_transaction_data(conn, client.id, request, new_balance)?;

    Ok(TransactionResponse {
        limit: client.limit,
        balance: new_balance,
    })
}

pub fn get_extract(conn: &Connection, client_id: u32) -> Result<ExtractResponse, ErrorResponse> {
    let client = get_client(conn, client_id)?;

    Ok(get_extract_data(conn, client_id, client)?)
}

fn get_new_balance(request: &TransactionRequest, client: &ClientData) -> Option<i64> {
    let new_balance = match request.transaction_type {
        TransactionType::Debit => {
            let new_balance = client.balance + (client.limit as i64 - request.value as i64);

            if new_balance < 0 {
                return None;
            } else {
                client.balance - request.value as i64
            }
        }
        TransactionType::Credit => client.balance + request.value as i64,
    };

    Some(new_balance)
}

fn get_client(conn: &Connection, client_id: u32) -> Result<ClientData, ErrorResponse> {
    let client = conn
        .query_row(
            "SELECT
               balance,
               debit_limit
             FROM clients
             WHERE id = (?1);",
            [client_id],
            |row| {
                Ok(ClientData {
                    id: client_id,
                    balance: row.get(0)?,
                    limit: row.get(1)?,
                })
            },
        )
        .optional()?
        .ok_or_else(|| ErrorResponse::ClientNotFound(client_id))?;

    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_new_balance_1() {
        let client = ClientData {
            id: 0,
            balance: 0,
            limit: 2000,
        };

        let transaction = TransactionRequest {
            description: "Transação".to_owned(),
            value: 1000,
            transaction_type: TransactionType::Debit,
        };

        assert_eq!(get_new_balance(&transaction, &client), Some(-1000));
    }

    #[test]
    fn test_get_new_balance_2() {
        let client = ClientData {
            id: 0,
            balance: 10000,
            limit: 2000,
        };

        let transaction = TransactionRequest {
            description: "Transação".to_owned(),
            value: 1000,
            transaction_type: TransactionType::Debit,
        };

        assert_eq!(get_new_balance(&transaction, &client), Some(9000));
    }

    #[test]
    fn test_get_new_balance_3() {
        let client = ClientData {
            id: 0,
            balance: 0,
            limit: 500,
        };

        let transaction = TransactionRequest {
            description: "Transação".to_owned(),
            value: 1000,
            transaction_type: TransactionType::Debit,
        };

        assert_eq!(get_new_balance(&transaction, &client), None);
    }

    #[test]
    fn test_get_new_balance_4() {
        let client = ClientData {
            id: 0,
            balance: 500,
            limit: 0,
        };

        let transaction = TransactionRequest {
            description: "Transação".to_owned(),
            value: 1000,
            transaction_type: TransactionType::Debit,
        };

        assert_eq!(get_new_balance(&transaction, &client), None);
    }

    #[test]
    fn test_get_new_balance_5() {
        let client = ClientData {
            id: 0,
            balance: -1000,
            limit: 2000,
        };

        let transaction = TransactionRequest {
            description: "Transação".to_owned(),
            value: 1000,
            transaction_type: TransactionType::Credit,
        };

        assert_eq!(get_new_balance(&transaction, &client), Some(0));
    }

    #[test]
    fn test_get_new_balance_6() {
        let client = ClientData {
            id: 0,
            balance: 0,
            limit: 2000,
        };

        let transaction = TransactionRequest {
            description: "Transação".to_owned(),
            value: 1000,
            transaction_type: TransactionType::Credit,
        };

        assert_eq!(get_new_balance(&transaction, &client), Some(1000));
    }
}

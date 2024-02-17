use async_sqlite::rusqlite::{Connection, OptionalExtension};
use chrono::Utc;

use crate::{models::*, routes::ErrorResponse};

pub fn add_transaction(
    conn: &Connection,
    client_id: u32,
    request: TransactionRequest,
) -> Result<TransactionResponse, ErrorResponse> {
    let client = get_client(conn, client_id)?;

    let new_balance = get_new_balance(&request, &client).ok_or(ErrorResponse::NotEnoughLimit)?;

    insert_transaction_data(conn, client_id, request, new_balance)?;

    Ok(TransactionResponse {
        limit: client.limit,
        balance: new_balance,
    })
}

fn insert_transaction_data(
    conn: &Connection,
    client_id: u32,
    request: TransactionRequest,
    new_balance: i64,
) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO transactions (
            client_id,
            value,
            type,
            description,
            date
         ) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            client_id,
            request.value,
            request.transaction_type,
            request.description,
            Utc::now(),
        ),
    )?;

    conn.execute(
        "UPDATE clients SET balance = (?1) WHERE id = (?2);",
        (new_balance, client_id),
    )?;

    Ok(())
}

pub fn get_extract(conn: &Connection, client_id: u32) -> Result<ExtractResponse, ErrorResponse> {
    let client = get_client(conn, client_id)?;

    Ok(get_extract_data(conn, client_id, client)?)
}

fn get_extract_data(
    conn: &Connection,
    client_id: u32,
    client: ClientData,
) -> anyhow::Result<ExtractResponse> {
    let mut query = conn.prepare(
        "SELECT value, type, description, date
            FROM transactions
         WHERE client_id = (?) 
         ORDER BY date DESC
         LIMIT 10;",
    )?;

    let transactions: Result<_, _> = query
        .query_map([client_id], |row| {
            Ok(TransactionData {
                value: row.get(0)?,
                transaction_type: row.get(1)?,
                description: row.get(2)?,
                date: row.get(3)?,
            })
        })?
        .collect();

    Ok(ExtractResponse {
        balance: ExtractData {
            total: client.balance,
            date: Utc::now(),
            limit: client.limit,
        },
        transactions: transactions?,
    })
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

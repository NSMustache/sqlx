use sqlx::mssql::MsSql;
use sqlx::{describe::Column, Executor};
use sqlx_test::new;

fn type_names(columns: &[Column<MsSql>]) -> Vec<String> {
    columns
        .iter()
        .filter_map(|col| Some(col.type_info.as_ref()?.to_string()))
        .collect()
}

#[sqlx_macros::test]
async fn it_describes_simple() -> anyhow::Result<()> {
    let mut conn = new::<MsSql>().await?;

    let d = conn.describe("SELECT * FROM tweet").await?;
    let columns = d.columns;

    assert_eq!(columns[0].name, "id");
    assert_eq!(columns[1].name, "text");
    assert_eq!(columns[2].name, "is_sent");
    assert_eq!(columns[3].name, "owner_id");

    assert_eq!(columns[0].not_null, Some(true));
    assert_eq!(columns[1].not_null, Some(true));
    assert_eq!(columns[2].not_null, Some(true));
    assert_eq!(columns[3].not_null, Some(false));

    let column_type_names = type_names(&columns);

    assert_eq!(column_type_names[0], "bigint");
    assert_eq!(column_type_names[1], "nvarchar(max)");
    assert_eq!(column_type_names[2], "tinyint");
    assert_eq!(column_type_names[3], "bigint");

    Ok(())
}
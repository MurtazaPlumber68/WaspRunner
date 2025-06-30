
use anyhow::Result;
use wasmrunner_core::registry::LocalRegistry;
use prettytable::{Table, Row, Cell};

pub async fn execute(all: bool) -> Result<()> {
    let registry = LocalRegistry::new()?;
    let containers = registry.list_containers(all).await?;
    
    if containers.is_empty() {
        println!("No containers found");
        return Ok(());
    }
    
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("CONTAINER ID"),
        Cell::new("IMAGE"),
        Cell::new("COMMAND"),
        Cell::new("CREATED"),
        Cell::new("STATUS"),
        Cell::new("NAMES"),
    ]));
    
    for container in containers {
        table.add_row(Row::new(vec![
            Cell::new(&container.id[..12]),
            Cell::new(&container.image),
            Cell::new(&container.command),
            Cell::new(&container.created),
            Cell::new(&container.status),
            Cell::new(&container.name),
        ]));
    }
    
    table.printstd();
    Ok(())
}

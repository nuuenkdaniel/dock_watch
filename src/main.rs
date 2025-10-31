use bollard::Docker;
use bollard::query_parameters::{ListContainersOptions, ListContainersOptionsBuilder};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let mut filters: HashMap<String, Vec<String>> = HashMap::new();
    filters.insert("status".to_string(), vec!["running".to_string()]);

    let opts: ListContainersOptions = ListContainersOptionsBuilder::default()
        .all(false)
        .filters(&filters)
        .build();

    let containers = docker.list_containers(Some(opts)).await?;
    dbg!(containers);

    Ok(())
}

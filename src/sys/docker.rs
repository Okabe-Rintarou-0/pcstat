use bollard::{errors::Error, service::ContainerInspectResponse, Docker};

lazy_static! {
    static ref DOCKER: Docker = Docker::connect_with_socket_defaults().unwrap();
}

pub async fn get_container_info(id: &str) -> Result<ContainerInspectResponse, Error> {
    let container = DOCKER.inspect_container(id, None).await;
    return container;
}

pub fn parse_container_pid(info: &ContainerInspectResponse) -> i64 {
    if let Some(ref state) = info.state {
        if let Some(pid) = state.pid {
            return pid;
        }
    }
    return -1;
}

pub fn parse_container_lower_dirs(info: &ContainerInspectResponse, container_lower_dirs: &mut Vec<String>) {
    if let Some(ref graph_driver) = info.graph_driver {
        let lower_dirs = graph_driver.data.get("LowerDir");
        if let Some(lower_dirs) = lower_dirs {
            for lower_dir in lower_dirs.split(":") {
                container_lower_dirs.push(lower_dir.to_string());
            }
        }
    }
}

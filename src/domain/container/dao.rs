use serde::Deserialize;

use crate::domain;

#[derive(Debug, Clone, Deserialize)]
pub struct InspectContainerParams {
    pub container_id: String,
}

/*
EXAMPLE
{
    "Id": "2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350",
    "Created": "2025-07-02T18:14:47.722748186Z",
    "Path": "/bin/bash",
    "Args": [
        "-c",
        "while true; do date; sleep 1; done"
    ],
    "State": {
        "Status": "running",
        "Running": true,
        "Paused": false,
        "Restarting": false,
        "OOMKilled": false,
        "Dead": false,
        "Pid": 1180549,
        "ExitCode": 0,
        "Error": "",
        "StartedAt": "2025-07-02T18:14:47.767992513Z",
        "FinishedAt": "0001-01-01T00:00:00Z"
    },
    "Image": "sha256:51e91de6f9a71d30eaab9cfccf10020a815f64e3eaaaeb9197b261017c14fc56",
    "ResolvConfPath": "/var/lib/docker/containers/2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350/resolv.conf",
    "HostnamePath": "/var/lib/docker/containers/2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350/hostname",
    "HostsPath": "/var/lib/docker/containers/2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350/hosts",
    "LogPath": "/var/lib/docker/containers/2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350/2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350-json.log",
    "Name": "/strange_pascal",
    "RestartCount": 0,
    "Driver": "overlay2",
    "Platform": "linux",
    "MountLabel": "",
    "ProcessLabel": "",
    "AppArmorProfile": "",
    "ExecIDs": null,
    "HostConfig": {
        "Binds": null,
        "ContainerIDFile": "",
        "LogConfig": {
            "Type": "json-file",
            "Config": {}
        },
        "NetworkMode": "bridge",
        "PortBindings": {},
        "RestartPolicy": {
            "Name": "no",
            "MaximumRetryCount": 0
        },
        "AutoRemove": false,
        "VolumeDriver": "",
        "VolumesFrom": null,
        "ConsoleSize": [
            0,
            0
        ],
        "CapAdd": null,
        "CapDrop": null,
        "CgroupnsMode": "private",
        "Dns": [],
        "DnsOptions": [],
        "DnsSearch": [],
        "ExtraHosts": null,
        "GroupAdd": null,
        "IpcMode": "private",
        "Cgroup": "",
        "Links": null,
        "OomScoreAdj": 0,
        "PidMode": "",
        "Privileged": false,
        "PublishAllPorts": false,
        "ReadonlyRootfs": false,
        "SecurityOpt": null,
        "UTSMode": "",
        "UsernsMode": "",
        "ShmSize": 67108864,
        "Runtime": "runc",
        "Isolation": "",
        "CpuShares": 1024,
        "Memory": 1073741824,
        "NanoCpus": 0,
        "CgroupParent": "",
        "BlkioWeight": 0,
        "BlkioWeightDevice": [],
        "BlkioDeviceReadBps": [],
        "BlkioDeviceWriteBps": [],
        "BlkioDeviceReadIOps": [],
        "BlkioDeviceWriteIOps": [],
        "CpuPeriod": 0,
        "CpuQuota": 0,
        "CpuRealtimePeriod": 0,
        "CpuRealtimeRuntime": 0,
        "CpusetCpus": "",
        "CpusetMems": "",
        "Devices": [],
        "DeviceCgroupRules": null,
        "DeviceRequests": null,
        "MemoryReservation": 0,
        "MemorySwap": 2147483648,
        "MemorySwappiness": null,
        "OomKillDisable": null,
        "PidsLimit": null,
        "Ulimits": [],
        "CpuCount": 0,
        "CpuPercent": 0,
        "IOMaximumIOps": 0,
        "IOMaximumBandwidth": 0,
        "MaskedPaths": [
            "/proc/asound",
            "/proc/acpi",
            "/proc/interrupts",
            "/proc/kcore",
            "/proc/keys",
            "/proc/latency_stats",
            "/proc/timer_list",
            "/proc/timer_stats",
            "/proc/sched_debug",
            "/proc/scsi",
            "/sys/firmware",
            "/sys/devices/virtual/powercap"
        ],
        "ReadonlyPaths": [
            "/proc/bus",
            "/proc/fs",
            "/proc/irq",
            "/proc/sys",
            "/proc/sysrq-trigger"
        ]
    },
    "GraphDriver": {
        "Data": {
            "ID": "2042abbae2afad3acb919e7d9b7b7c150f0ad9427fa73180ada0a81a6f0e3350",
            "LowerDir": "/var/lib/docker/overlay2/4c72f1909e1c6ae70ccdde31be76eed3ca1362f9dbc1e1721f4e85984d6179a9-init/diff:/var/lib/docker/overlay2/f6ed3044545efa4128cb4c256d0ce4150598a50f69b47567b152bf7722a3e36f/diff",
            "MergedDir": "/var/lib/docker/overlay2/4c72f1909e1c6ae70ccdde31be76eed3ca1362f9dbc1e1721f4e85984d6179a9/merged",
            "UpperDir": "/var/lib/docker/overlay2/4c72f1909e1c6ae70ccdde31be76eed3ca1362f9dbc1e1721f4e85984d6179a9/diff",
            "WorkDir": "/var/lib/docker/overlay2/4c72f1909e1c6ae70ccdde31be76eed3ca1362f9dbc1e1721f4e85984d6179a9/work"
        },
        "Name": "overlay2"
    },
    "Mounts": [],
    "Config": {
        "Hostname": "2042abbae2af",
        "Domainname": "",
        "User": "",
        "AttachStdin": false,
        "AttachStdout": false,
        "AttachStderr": false,
        "Tty": false,
        "OpenStdin": false,
        "StdinOnce": false,
        "Env": [
            "[]",
            "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
        ],
        "Cmd": [
            "/bin/bash",
            "-c",
            "while true; do date; sleep 1; done"
        ],
        "Image": "batchman-forever:latest",
        "Volumes": null,
        "WorkingDir": "",
        "Entrypoint": null,
        "OnBuild": null,
        "Labels": {
            "org.opencontainers.image.ref.name": "ubuntu",
            "org.opencontainers.image.version": "24.04"
        }
    },
    "NetworkSettings": {
        "Bridge": "",
        "SandboxID": "0949c4d13523fbff0c86185366a789c248d874ab759da623f73b96b53d6242a2",
        "SandboxKey": "/var/run/docker/netns/0949c4d13523",
        "Ports": {},
        "HairpinMode": false,
        "LinkLocalIPv6Address": "",
        "LinkLocalIPv6PrefixLen": 0,
        "SecondaryIPAddresses": null,
        "SecondaryIPv6Addresses": null,
        "EndpointID": "b7cafe5ba4047169ce48c69a989cf4f1c7f00b50ae5addc3a693b39fc5c13e38",
        "Gateway": "172.17.0.1",
        "GlobalIPv6Address": "",
        "GlobalIPv6PrefixLen": 0,
        "IPAddress": "172.17.0.2",
        "IPPrefixLen": 16,
        "IPv6Gateway": "",
        "MacAddress": "16:3c:13:e0:f8:aa",
        "Networks": {
            "bridge": {
                "IPAMConfig": null,
                "Links": null,
                "Aliases": null,
                "MacAddress": "16:3c:13:e0:f8:aa",
                "DriverOpts": null,
                "GwPriority": 0,
                "NetworkID": "33fe75aa1e9518de79000ad231e3a039cf35e90cae317e0efee88c128768b905",
                "EndpointID": "b7cafe5ba4047169ce48c69a989cf4f1c7f00b50ae5addc3a693b39fc5c13e38",
                "Gateway": "172.17.0.1",
                "IPAddress": "172.17.0.2",
                "IPPrefixLen": 16,
                "IPv6Gateway": "",
                "GlobalIPv6Address": "",
                "GlobalIPv6PrefixLen": 0,
                "DNSNames": null
            }
        }
    }
}
*/
#[derive(Debug, Clone, Deserialize)]
pub struct InspectContainerResult {
    #[serde(rename = "State")]
    pub state: ContainerState,

    #[serde(rename = "LogPath")]
    pub log_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum LogType {
    #[serde(rename = "json-file")]
    JsonFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub _log_type: LogType,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HostConfig {
    #[serde(rename = "LogConfig")]
    pub _log_config: LogConfig,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct ContainerState {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Running")]
    pub running: bool,
    #[serde(rename = "Paused")]
    pub paused: bool,
    #[serde(rename = "Restarting")]
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    #[serde(rename = "Dead")]
    pub dead: bool,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RunContainerParams {
    pub task_definition: domain::task_definition::entities::task_definition::Model,
}

#[derive(Debug, Clone)]
pub struct RunContainerResult {
    pub container_id: String,
}

#[derive(Debug, Clone)]
pub struct KillContainerParams {
    pub container_id: String,
}

#[derive(Debug, Clone)]
pub struct StopContainerParams {
    pub container_id: String,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone)]
pub struct RemoveContainerParams {
    pub container_id: String,
    pub force: bool,
    pub remove_volumes: bool,
    pub remove_links: bool,
}

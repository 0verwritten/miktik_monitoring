# Mikrotik monitoring software
Early version of monitoring tool. Will be ready to use soon

### Links you may want to visit:
- Last changes              [here](./CHANGELOG.md)
- `commands.json`           [manual](#commandsjson-explanation)
- `credentials.json`        [explanation](#credentialsjson-explanation)
- `grafana_dashboard.json`  beta version of [dashboard](./grafana_dashboard.json)

### `commands.json` explanation
This is file where all commands application will execute automaticaly are located. Here is example and a bit of explanation:
#### **Important**:
Application adds `miktik_` to every name

```
{
    "commands":[                                    <= array of commands
        {
            "command": "/ping",                     <= ( required ) command name
            "name": "ping_test",                    <= ( required ) indentifier used in prometheus to indentify results ( application will add `miktik_` before )
            "attributes": [ "host" ],               <= what attributes to seek ( displayed along with command indentifier)
                                                        if there is no such parameter, then application will display all responces
                                                        if you haven't put anything in it then nothing will be displayed as parameter
            "graph_targets": ["time"],              <= this parameter application should print to prometheus as value ( should be an number )
            "split_character": "ms",                <= separator to split value into several responces
            "split_attributes": ["time"],           <= Attributes where application should split responce in halves
            "query": [                              <= additional queries for command
                "=count=1", 
                "=address=1.1.1.1" 
                ]
        },
        {
            "command":"/ip/cloud/print",
            "name": "address_v6_info",
        }
    ]
}
```

### `credentials.json` explanation
This file contains all information about routerboars application needs except certificates if you use ones:

```
[
    {
        "name": "Router name",          <= Alias of your routerboard
        "uri": "192.168.31.1:8729",     <= Routerboard's ip address
        "use_ssl": true,                <= `true` if you use ssl ( recommended ), `false` if no
        "username": "user1",            <= Username
        "password": "123",              <= Password
        "cert": "/path/to/file"         <= Certificate file location
    }
]
```

More information in soon

[<img src="./templates/images/gears.gif" width="250"/>](./templates/images/gears.gif)
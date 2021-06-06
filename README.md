# mikrotik_monitoring
Early version of monitoring tool. It doesn't have promtheus integration yet. For now it only have plain and encrypted router connection with user-typed commands

### Links you might want to visit:
- Last changes read         [here](./CHANGELOG.md)
- `commands.json`           [manual](#commands.json-explanation)
- `credentials.json`        [explanation](#credentials.json-explanation)

### `commands.json` explanation
This is file where all commands application will execute automaticaly are located. Here is example and a bit of explanation:

```
{
    "commands":[                                    <= array of commands
        {
            "command":"/ip/address/print",          <= command name
            "multiple_objects": true,               <= if application should look for multiple results
            "name": "address_info",                 <= indentifier used in prometheus to indentify results
            "attributes": [                         <= what attributes to seek ( displayed along with command indentifier)
                    "network",
                    "comment",
                    "address",
                    "interface"

            ],
            "graph_targets": []                     <= which parameter application should print to prometheus as value ( should be an number )
        },
        {
            "command":"/ip/cloud/print",
            "multiple_objects": false,
            "name": "address_v6_info",
            "attributes": [
                    "public-address-ipv6",
                    "ddns-enabled",
                    "dns-name",

            ],
            "graph_targets": []
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
        "uri": "192.168.31.1",          <= Routerboard's ip address
        "use_ssl": true,                <= `true` if you use ssl ( even without certificate ), `false` of no
        "username": "user1",            <= Username
        "password": "123",              <= Password
        "cert": "/path/to/file"         <= Certificate file location
    }
]
```

More information on development

[<img src="./templates/images/gears.gif" width="250"/>](./templates/images/gears.gif)
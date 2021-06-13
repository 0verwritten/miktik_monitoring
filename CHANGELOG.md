# Change log + tasks
This is file where I describe all the [changes])(#сhages) I've done so far and [tasks](#to-do) I'm going to make

## TO DO:

#### Errors:

#### New features:
- [ ] ssl ca verification 
- [ ] ssl certificate acceptance
- [ ] grafana dashboard
- [ ] prometheus authentication
- [ ] add posibility to disable or change automatic prependix

##### Big inverntions:
- [ ] Improve performance
- [ ] not panic when router not responds
- [ ] reconnect to router on an error
- [ ] custum function to that implement aditional functionality
- [ ] async requests
- [ ] upgrade `README.md` file
- [ ] Verbose and non-verbore loging update
- [ ] class that parses certain responces ( may have to be marked to parse )

##### Light updates:
- [ ] update commands template and example files
- [x] not display value if it isn't present in the responce

#### Not that important:
- [ ] use router name from `credentials.json` to indentify routers
- [ ] utf8 error handling ( never happened so far )
- [ ] connect to routerboards using domain name
- [ ] make config change on site?
- [ ] make error type, not just a string
- [ ] Optimise responce parsing using custum types for each query
- [ ] parse `uptime` field to display beautifully
- [ ] update trap error

<!-- 
#### Older tasks:
- [x] "no `graph_targets` to no results" - bug
- [x] requests handling correctly
- [x] application frezes after invalid commands
- [x] operate with router errors
- [x] bug when responce is too long
- [x] strange responce from application ( variables mess up and application freezes sometimes )
- [x] make config file and gradana dashboard using prometheus data got by this application
- [x] command query execution from `commands.json` file
- [x] make config for users credentials
- [x] add router custum naming in login config file
- [x] Write config tutorial in README.md
- [x] make `query_teller` being able to operate with list of connections
- [x] rename functions to make them more meaningful
- [x] custum configuration of web server from main function
- [x] queries live update or button to update
- [x] make some comments + function documentation
- [x] remove length mark or use it correctly in router's reply
- [x] update commands config file
- [x] commands config implementation in lib
- [x] do all mikrotik query types
- [x] cover mikrotik responce message commands
- [x] prometheus integration
- [x] encrypt user credentials
- [x] uft8 converter error ( some characters appears in the end of the responce or just disapear ... )
- [x] turn `tell_get` into functuins that borows mutable vector to push responces inside it
- [x] move enerything into a library
- [x] ssl encryption 
-->

## Chages:

### Last changes:
- Changed Hashmap responce style to Vector one ( responces with vector of ready-to-use strings instead of vector of hashmaps)
- Created first version of grafana dashboard
- Added automatic prependix in web page
- Split option in `commands.json` file
- Updated `Readme.md` slightly
- Updated grafana dashboard
- Updated `commands.json`file


### Older Changes: 
##### v0.1.4
- Fixed "panic on !trap sign" gate
- Made new reading algorithm
- Fixed bug - last line wasn't displayed
- updated `commands.json` file
- First version of verbose option

##### v0.1.3
- added config files explanation
- changed files location slightly
- moved routerboards users into separate file to seek identities from

##### v0.1.2
- reorganised todo list
- slightly reorganised files organization
- made Readme.md more prety
- made `query_teller` being able to operate with list of connections
- queries live update or button to update ( on the web page ) 
- custum configuration of web server from main function

##### v0.1.1
- moved to tiny_http
- first version of http post page
- cleaned everything up a little bit

##### v0.1.0
- json file to seek commands in automaticly
- slight commands config file
- added some comments
- first test of asyncing
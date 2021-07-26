# Change log + tasks
This is file where I describe all the [changes])(#сhages) I've done so far and [tasks](#to-do) I'm going to make

## TO DO:

#### Errors:
- [x] utf8 error handling ( never happened so far )

#### New features:
- [x] grafana dashboard
- [ ] prometheus authentication

##### Big inverntions:
- [x] docker image
- [ ] release a release
- [x] upgrade `README.md` file
- [ ] Verbose and non-verbore loging update

##### Light updates:
- [x] update commands template and example files
- [x] clean up the code
- [ ] update interactive mode ( add authorization in console, ability to choose where to take credentials from )

#### Not that important:
- [ ] verbosibility levels
- [ ] split config into sections to choose from in cerdentials.json config file
- [ ] docker container initialization process ( when mounting volumes with config file from outside )
- [ ] add posibility to disable or change automatic prependix
- [ ] custum function to that implement aditional functionality
- [x] use router name from `credentials.json` to indentify routers
- [x] connect to routerboards using domain name
- [ ] make config change on site?
- [ ] make error type, not just a string
- [ ] Optimise responce parsing using custum types for each query
- [ ] parse `uptime` field to display beautifully

<!-- 
#### Older tasks:
- [x] ability to use output from previous commands as query for next ones
- [x] reconnect to never connected routerboards
- [x] add routerboard status ( up/down )
- [x] ssl ca verification 
- [x] ssl certificate acceptance
- [x] Colored display
- [x] parallel requests
- [x] update `!trap` error
- [x] don't panic when router doen't respond
- [x] reconnect to router on an error
- [x] reconnection function 
- [x] not display value if it isn't present in the responce
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
- Updated grafana dashboard
- made docker image
- added possiblity to use domain name instead of ip address
- switched to using router name from `credentials.json` to indentify routers

### Older Changes: 
##### v0.2.2
- fixed dropping connection error
- added reconnection for never connected devices
- changed the way responce is processing
- utf8 error handling
- added ability to use output from previous commands as query for next ones
- fixed dec_to_hex converter

##### v0.2.1
- updated dashboard
- added status of routerboard
- made automatic metrics collector sending requests in parallel
- fixed reconnecting bug
- login faliue fix
- updated parallel requesting

##### v0.2.0
- added certificate verification ( by authority, certificate itself, no verification at all )
- added function to reconnect to router
- updated error responces
- added automatic reconnection to router
- add skipping router on initial in case of error
- errors now print into erorr stream ( better for logging )
- added some colors to output

##### v0.1.5
- Changed Hashmap responce style to Vector one ( responces with vector of ready-to-use strings instead of vector of hashmaps)
- Created first version of grafana dashboard
- Added automatic prependix in web page
- Split option in `commands.json` file
- Updated `Readme.md` slightly
- Updated grafana dashboard
- Updated `commands.json`file

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
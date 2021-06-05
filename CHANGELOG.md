# Change log + tasks
This is file where I describe all the [changes])(#сhages) I've done so far and [tasks](#to-do) I'm going to make

## TO DO:

#### Errors:
- [ ] utf8 error handling
- [ ] "no `graph_targets` to no results" - bug
- [ ] reqults handling correctly

#### New features:

##### Big inverntions:
- [ ] ssl ca verification 
- [ ] ssl certificate acceptance
- [ ] make config for users credentials
- [ ] add router custum naming in login config file
- [ ] make config file and gradana dashboard using prometheus data got by this application
- [x] make `query_teller` being able to operate with list of connections
- [ ] upgrade `README.md` file
- [ ] Verbose and non-verbore loging update

##### Slight updates:
- [x] rename functions to make them more meaningful
- [ ] Write config tutorial in README.md
- [ ] update commands template and example files
- [x] custum configuration of web server from main function
- [x] queries live update or button to update

#### Not that important:
- [ ] make error type, not just a string
- [ ] Optimise responce parsing using custum types for each query ( not important )
- [ ] async requests or parallel ( not that important if one server is knocking, so not in near future )

#### Older tasks:
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

## Chages:

### Last changes:
- reorganised todo list
- slightly reorganised files organization
- made Readme.md more prety
- made `query_teller` being able to operate with list of connections
- queries live update or button to update ( on the web page ) 
- custum configuration of web server from main function


### Older Changes: 
##### v0.1.1
- moved to tiny_http
- first version of http post page
- cleaned everything up a little bit

##### v0.1.0
- json file to seek commands in automaticly
- slight commands config file
- added some comments
- first test of asyncing
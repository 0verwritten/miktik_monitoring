# mikrotik_monitoring
Early version of monitoring tool. It doesn't have promtheus integration yet. For now it only have plain and encrypted router connection with user-typed commands

TO DO:

- [x] make some comments + function documentation
- [x] remove length mark or use it correctly in router's reply
- [ ] async requests or parallel ( not that important if one server is knocking, so not in near future )
- [x] move enerything into a library
- [x] ssl encryption
- [ ] rename functions to make them more meaningful
- [ ] ssl ca verification 
- [ ] ssl certificate acceptance
- [ ] make error type, not just a string
- [ ] utf8 error handling
- [x] do all mikrotik query types
- [x] cover mikrotik responce message commands
- [x] prometheus integration
- [x] encrypt user credentials
- [x] uft8 converter error ( some characters appears in the end of the responce or just disapear ... )
- [ ] Optimise responce parsing using custum types for each query
- [ ] Write config tutorial in README.md
- [ ] make config for users credentials
- [x] update commands config file
- [x] commands config implementation in lib
- [ ] update commands template and example files
- [x] turn `tell_get` into functuins that borows mutable vector to push responces inside it
- [ ] add router custum naming in login config file
- [ ] "no `graph_targets` to no results" - bug
- [ ] make config file and gradana dashboard using prometheus data got by this application

### Last changes:
- moved to tiny_http
- first version of http post page
- cleaned everything up a little bit


### Older Changes: 
- json file to seek commands in automaticly
- slight commands config file
- added some comments
- first test of asyncing
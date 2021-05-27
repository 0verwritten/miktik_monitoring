# mikrotik_monitoring
Early version of monitoring tool. It doesn't have promtheus integration yet. For now it only have plain and encrypted router connection with user-typed commands

TO DO:

- [ ] make some comments + function documentation
- [x] remove length mark or use it correctly in router's reply
- [ ] async requests or parallel
- [x] move enerything into a library
- [x] ssl encryption
- [ ] rename functions to make them more meaningful
- [ ] ssl ca verification 
- [ ] ssl certificate acceptance
- [ ] make error type, not just a string
- [ ] utf8 error handling
- [x] do all mikrotik queriy types
- [ ] cover mikrotik responce message commands
- [ ] prometheus integration
- [x] encrypt user credentials
- [x] uft8 converter error ( some characters appears in the end of the responce or just disapear ... )
- [ ] Optimise responce parsing using custum types for each query

Last changes:
- json file to seek commands in automaticly

Didland

Example usage:
```
    Usage:
        didland <command> <args>
        didland init
        didland did
        didland doc
        didland connect  <did name> <did>

    Basic Didcomm Messaging:
        didland write    <to did name> <message>  -->  <dcem>
        didland read     <dcem>                   -->  <from did name> <message>

    Verifiable Credentials:
        didland issue Passport         <to did name>  -->  <dcem>
        didland issue DriversLicense   <to did name>  -->  <dcem>
        didland issue TrafficAuthority <to did name>  -->  <dcem>
        didland issue LawEnforcer      <to did name>  -->  <dcem>

        didland hold    <credential name> <dcem>
        didland present <credential name> <to did name>  -->  <dcem>
        didland verify  <issuer did name> <dcem>

    View stored data:
        didland messages
        didland credentials
        didland credential <credential name>
        didland presentations
        didland presentation <presentation name>
```

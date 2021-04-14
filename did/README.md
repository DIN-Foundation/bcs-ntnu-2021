did CLI

Example usage:
```
    Usage:
        did <command> <args>
        did init
        did did
        did doc
        did connect  <did name> <did>

    Basic Didcomm Messaging:
        did write    <to did name> <message>  -->  <dcem>
        did read     <dcem>                   -->  <from did name> <message>

    Verifiable Credentials:
        did issue Passport         <to did name>  -->  <dcem>
        did issue DriversLicense   <to did name>  -->  <dcem>
        did issue TrafficAuthority <to did name>  -->  <dcem>
        did issue LawEnforcer      <to did name>  -->  <dcem>

        did hold    <credential name> <dcem>
        did present <credential name> <to did name>  -->  <dcem>
        did verify  <issuer did name> <dcem>

    View stored data:
        did messages
        did credentials
        did credential <credential name>
        did presentations
        did presentation <presentation name>
```

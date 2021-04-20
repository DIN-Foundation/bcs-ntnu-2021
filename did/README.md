did CLI

Example usage:
```
    Basic:
        did
        did doc
        did connect <connection id> <did>

    DIDComm v2 messaging:
        did write  <connection id> <message>  -->  <dcem>
        did read   <dcem>                     -->  <message id>

    DIDComm v2 + Verifiable Credentials:
        did issue   Passport         <connection id>  -->  <dcem>
        did issue   DriversLicense   <connection id>  -->  <dcem>
        did issue   TrafficAuthority <connection id>  -->  <dcem>
        did issue   LawEnforcer      <connection id>  -->  <dcem>
        did hold    <dcem>                            -->  <credential id>
        did present <credential id>  <connection id>  -->  <dcem>
        did verify  <issuer connection id> <subject connection id> <dcem>  -->  <presentation id>

    View stored data:
        did messages
        did message <message id>
        did connections
        did connection <connection id>
        did credentials
        did credential <credential id>
        did presentations
        did presentation <presentation id>
```

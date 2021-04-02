didchat - Second attempt at a didcomm-rs based CLI chat app.

```
$ didchat . help

    Usage:
        didchat <path> <command>
        
        didchat <path> init
        didchat <path> doc    
        didchat <path> did    
        didchat <path> messages

        didchat <path> connect <name> <did>

        didchat <path> write <name> <message>      -->  <encrypted message>
        didchat <path> read <encrypted message>    -->  <name> <message>

    Example - Write to self:
        didchat . init
        didchat . connect self $(didchat . did)
        didchat . read $(didchat . write self "Hello self!")

    Example - Write to peer:
        didchat jonas init
        didchat snorre init
        
        didchat snorre connect jonas $(didchat jonas did)
        didchat jonas connect snorre $(didchat snorre did)

        didchat jonas read $(didchat snorre write jonas "Hello Jonas. How are you?")
        didchat snorre read $(didchat jonas write snorre "Hi Snorre:) I have seen better days.")

```
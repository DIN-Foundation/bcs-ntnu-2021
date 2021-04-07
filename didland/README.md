Didland 

Example basic usage:
```
    Example - Write to self:
        didland . init
        didland . write self \"Hello self!\"
        didland . read $(didland . write self \"How do you do?\")

        didland . write self \"I am very well, thank you :+1:\" > hello.dcem
        didland . read $(cat hello.dcem)

    Example - Write to peer:
        didland jonas init
        didland snorre init

        didland snorre connect jonas $(didland jonas did)
        didland jonas connect snorre $(didland snorre did)

        didland jonas read $(didland snorre write jonas \"Hello Jonas. How are you?\")
        didland snorre read $(didland jonas write snorre \"Hi Snorre:) I have seen better days.\")
```
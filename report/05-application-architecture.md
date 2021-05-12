# 5 Application Architecture



## 5.x Agent database

### 5.x.x The `.did/` directory

The database is contained within the `.did/` directory created when initializing the agent.

@TODO Add screenshot


### 5.x.x Messages

### 5.x.x Connections

### 5.x.x Portability

The agent database, represented by the `.did/` directory, should be portable. A user should be able to move it to any other location on local machine, or to any other machine, and the agent should still work.




## 5.x Multiple agents

### 5.x.x One directory per agent

```

for i in bob lisa snorre jonas
do
	mkdir $i;
	cd $i;
	did init;
	cd ..;
done
```

@TODO Add screenshot

### 5.x.x Communicating by sharing files

```
mkdir bob/ lisa/
cd bob/
did init
did did self > ../bob.did

cd ../lisa
did init
did did self > ../lisa.did

cd ../lisa
cat ../bob.did | did connect bob
cd ../bob
cat ../lisa.did | did connect lisa
```

@TODO Add screenshot




## 5.x Unix design

- We want the CLI to be easy to integrate with existing Unix tools

```
cat message.dcem | did read | grep jonas 
```


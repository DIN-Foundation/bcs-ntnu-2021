# 1. Mål og rammer

## 1.1 Bakgrunn

*See [project-description.md](./project-description.md)*

## 1.2 Prosjektmål

Develop a proof-of-concept SSI application, and use the proof-of-concept to discover and present strengths and weaknesses of the current SSI infrastructure, with a focus on interoperability. 

There are two key areas of interoperability:
1. Interoperability between different did-methods - How and where DIDs are stored.
2. Interoperability between different wallets/agents - Where ownership of DIDs are prooved and where VCs are issued, stored, communicated and verified.

## 1.3 Rammer

The BSc project runs between `01.feb.2021` and `20.may.2021`.

The BSc project will be developed as an open source project on the DIN-Foundation's Github page here: https://github.com/DIN-Foundation/bsc-ntnu-2020/. 

See https://www.din.foundation/ for more info about the DIN-foundation.


# 2. Omfang

## 2.1 Fagområde

The project is developed within the field of Self-sovereign-identity - SSI.

Here are some links to get started:
- DIDs (Decentralized identifiers) by W3C: https://www.w3.org/TR/did-core/
- VC (Verifiable credentials) by W3C: https://www.w3.org/TR/vc-data-model/
- Working standards under DIF working groups: https://identity.foundation/#wgs

Also see [project-description.md](./project-description.md#Background) for more info.

## 2.2 Avgrensing

The project is looking at interoperability of the current SSI-infrastructure. More specifically interoperability between wallets, and interoperability between DID-methods.

To understand interoperability a proof-of-concept application will be developed. The proof-of-concept will implement one specific use-case of SSI. 

The proof-of-concept should implement the following parts of an SSI-workflow:
- Create an issuing-agent linked to a DID.
- Create a holder-agent linked to a DID.
- Create a verifying-agent linked to a DID.
- Issue a credential to a holder.
- Enable holder to present credential to a verifier.
- Enable verifier to verify credential presented by holder.

A limited set of ledgers and wallets should be investigated:

**@TODO Make list of wallets to investigate**

**@TODO Make list of ledgers to investigate**

## 2.3 Oppgavebeskrivelse

*See [project-description.md](./project-description.md)*

### User story

*Disclaimer: The BSc has no affiliation with Statens Vegvesen at the moment of writing, and the story below is pure fantasy.*

The norwegian driver license issuer, Statens Vegvesen, is considering to start issuing it's driver licenses as verifiable credentials (VCs). Statens Vegvesen is not sure if verifiable credentials is the future yet, but are willing to try and dip it's toes in the water. Statens vegvesen will still issue credentials in the traditional way for the forseeable future.

What Statens Veivesen want is a proof-of-concept SSI-application which will issue, hold and verify driver-licenses. They are hoping that this will bootstrap SSI in Norway, as this will enable other individuals and organizations to start experimenting with holding and verifying a serious credential which actually is useful.

The proof-of-concept may demonstrate that a driver license as a verifiable credential could be considered on par, legally speaking, with traditional driver licenses.

Statens Vegvesen wants the application to follow open standards which will enable the application to be agnostic about where and how credentials are issued, stored and verified. In other words Statens Vegvesen want to avoid the application to be locked to a specific ledger and a specific wallet.


### Deliverables

- Proof-of-concept SSI application which implements the user-story
- A comparison matrix of ledger interoperability
- A comparison matrix of wallet interoperability
- Project report

# 3. Prosjektorganisering

## 3.1 Ansvarsforhold og roller

| Navn        | Role                                                            |
|-------------|-----------------------------------------------------------------|
| Jonas       | Developer                  (Service Delivery Manager in Kanban) |
| Snorre      | Client and domain expert 1 (Service Request Manager in Kanban)  |
| Mariusz     | Domain expert 2                                                 |
| Abylay      | Domain expert 3                                                 |
| Deepti      | Academic supervisor                                             |

## 3.2 Rutiner og regler i gruppa

### Developer rules

- Rule 1: To keep the project moving at all times, work on the project every day of the week. Minimum 5 minutes, maximum 8 hours.
- Rule 2: Average 30 hours each week.
- Rule 3: Respond to NTNU-mail, Discord-messages, Github-comments, Phone-calls, SMS in less than 24 hours.
- Rule 4: Write meeting-notes for every meeting.
- Rule 5: Log all hours in Toggl under these 5 labels: [Meeting, Organizing, Writing, Coding, Researching]. These labels is not based on any standard, and is just something the author has found useful based on experience.
- Rule 6: All documents relevant to the project, should be found in the Github repository, unless it contains sensitive/private information.

# 4. Planlegging, oppfølging og rapporter

## 4.1 Hovedinndeling av prosjektet

The project is loosely driven forward by the Kanban framework. To keep track of progress in a Kanban project the Kanban board is essential. See the board here: https://github.com/DIN-Foundation/bsc-ntnu-2020/projects/1.

Our Kanban-board has 5 columns:

**1. Backlog**

List of tasks which have been thought of, but is lacking details like what needs to be done and when the implementation is supposed to happen.

**2. Todo**

List of tasks that are planned to be implemented in the near future, and have enough details ironed out to make it possible to start the implementation.

**3. In Progress**

List of tasks that are in progress. A developer have written words or code. These tasks should ideally be linked to an open Pull Request on Github with a WIP label on it.

**4. In Review**

List of tasks where the developer has implemented the task, and is waiting to get a stamp of approval from a second pair of eyes. Approval could be given by the client, supervisor or any of the domain experts. The developer will request review from the specific person which is considered best suited to review the pull request. This specific person will be notified about this on Discord, and via email if necessary.

**5. Done**

List of task where the pull request linked to the task has been approved in merged into the main branch of the git source tree.


### Why Kanban instead of Scrum?

Kanban is a lightweight/low-overhead framework. Since the developer-team consist of only 1 person, Scrum is considered too much overhead, since all the Scrum rituals has to be organized and executed by a single person.

Scrum has a focus on delivering something every sprint. Kanban does not say anything about how often something should be delivered. Since this project is research-heavy, there is still a big question-mark about what should be made, and when something should be made. Of course it would be possible to schedule a demo every 2 weeks to show... something. The feeling here is that we need more flexibility than that. 

A demo will be held on the weekly meetings showcasing what has been done the last week, if something worth showing has been done. This would be similar to having a sprint of 1 week in Scrum. It could be safe to say that we are having Kanban with 1 week sprints. But the scheduling of new tasks and demonstration of completed tasks happens in the same meeting VS being 2 separate meetings (often on 2 separate days - Demo-meeting on last day of sprint, planning-meeting on first day of next sprint) in Scrum.


## 4.2 Plan for statusmøter og beslutningspunkter i perioden

We have planned 2 weekly meetings, 1 with supervisor and 1 with client and domain experts.

- Client and Domain experts meeting: Tuesdays @ 12:30
- Academic supervisor meeting: Tuesdays @ 13:00

Participants of these meetings have received invites by email, to make it easy to add the scheduled meetings to their respective calendars.

# 5. Organisering av kvalitetssikring

## 5.1 Dokumentasjon, standardbruk og kildekode

The language used to develop the proof-of-concept app as not been decided yet, but we would like to use a language with mature tools for documentation, testing, linting, formatting. These tools should be run by the CI on every commit to the source-tree to ensure consistent quality.

Here are some key areas of quality control, which can be expected:

**API documentation generated from code docstrings**

This is a very useful tool to document interfaces in the code bases. Either interfaces between different modules in the code-base, or external interfaces like a CLI or HTTP API etc.

**High level visual documentation**

Some high-level documentation will be required to explain the architecture of the entire solution, using diagrams.

**Testing of API's**

There will be an interface to control the proof-of-concept app. Most of the testing we do should focus heavily on testing this interface, be it a CLI or a HTTP API or both or something else. How this interface will look is TBD.

**@TODO: Define API of proof-of-concept app**

**Linting and formatting using community defaults and best practices**

Programming communities have standards and default practices regarding how to write code. Automated tools like a linter should run in the CI environment to ensure that these best practices are being followed.


## 5.2 Konfigurasjonsstyring

### OS and platform support

The proof-of-concept application will most likely be a mix between an CLI and a Web-app - TBD. The details of the app architecture is yet to be figured out. As a general principle, we should try to be cross-browser and cross-shell, which will in turn make us cross-OS. This is in the spirit of the project, as we are trying to measure interoperability in the SSI-space. Practice as you preach.

**Shells to support:**
- [zsh](https://en.wikipedia.org/wiki/Z_shell) - Default shell on MacOS
- [bash](https://en.wikipedia.org/wiki/Bash_(Unix_shell)) - Default shell on most Linux distributions

**Browsers to support:**
- Chromium (Chrome, IE, Opera, Brave, etc.) - Most used browser technology in the world
- Firefox - Default browser on most Linux distributions
- Safari - Default browser on MacOS

### Collaboration Tools

A list of tools required participate in the project:

- [Git](https://git-scm.com/) - Version control
- [Github](https://github.com/) - Host files, issues, pull-requests
- [Discord](https://discord.com/) - Direct messaging
- [Google Meet](https://meet.google.com/) - For domain experts meeting
- [MS Teams](https://www.microsoft.com/en/microsoft-teams/group-chat-software) - For academic supervisor meeting

### [TBD] Development environment, programming languages, frameworks

The initial phase of the project will be spent trying to figure out this part, together with the overall architecture of the application. A general principle here is that we want to build on top of an existing codebase. We do not want to start from scratch.

- Hyperledger Aries has a nice overview over different frameworks that can be used to develop SSI-agents in different languages. See: https://github.com/hyperledger/aries#aries-agent-frameworks.
- Trinsic also mentions `Sidetree`, `Universal Resolver` and `KERI` in their overview, under the DIF codebases section. See: https://trinsic.id/open-source-ssi-codebases/.


## 5.3 Risikoanalyse (identifisere, analysere, tiltak, oppfølging)

### Risk 1: Proof-of-concept app development-cost unpredictability

There is a big risk that the time it takes to develop the proof-of-concept app will be very unpredictable. Most of the development happens in an area of programming which is completely new to the developer of the app. This makes the learning curve steep, and the time-estimate more upredictable. Hopefully the developer will draw from his experience of working in other fields, and adapt quickly, but it is hard to know exactly how much time will be spent to adapt to the new development environment.

**Risk mitigation strategy**
The weekly domain-experts meeting is an important arena to show progress, and to discover delays in progress. In this meeting we will adjust our strategy to try and stick to the schedule. There should be a dedicated agenda point in this meeting to address this risk during the implementaiton-phase of the app.

# 6. Plan for gjennomføring

The project starts in Week 5 (01.feb - 07.feb) and ends in Week 20 (17.may - 23.may). The deadline for delivering the project is 20.may.

### Timespan
- 15 weeks and 3 days
- 108 days
- 78 weekdays
- 624 weekdayhours (8 hours per weekday)

### Gantt chart

| Milestones                          |Week 5|Week 6|Week 7|Week 8|Week 9|Week 10|Week 11|Week 12|Week 13|Week 14|Week 15|Week 16|Week 17|Week 18|Week 19|Week 20|
|-------------------------------------|------|------|------|------|------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|
| High-level design doc               |  x   |  x   |      |      |      |       |       |       |       |       |       |       |       |       |       |       |
| Learn devenv                        |      |  x   |  x   |      |      |       |       |       |       |       |       |       |       |       |       |       |
| Proof of concept app implementation |      |      |  x   |   x  |  x   |   x   |   x   |   x   |   x   |   x   |       |       |       |       |       |       |
| DID-method interop investigation    |      |      |      |      |      |       |       |   x   |   x   |   x   |   x   |       |       |       |       |       |
| Wallet interop investigation        |      |      |      |      |      |       |       |       |   x   |   x   |   x   |   x   |       |       |       |       |
| Research SSI                        |  x   |  x   |  x   |   x  |  x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |       |       |       |
| Bachelor Report                     |  x   |  x   |  x   |   x  |  x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |   x   |  x    |   x   |   x   |

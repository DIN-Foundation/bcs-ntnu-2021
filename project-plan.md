# 1. Mål og rammer

## 1.1 Bakgrunn

*See [project-description.md](./project-description.md)*

## 1.2 Prosjektmål

Develop a proof-of-concept SSI application, and use the proof-of-concept to discover and present strengths and weaknesses of the current SSI infrastructure, with a focus on interoperability. 

There are two key areas of interoperability:
1. Interoperability between different ledgers - where DIDs and VCs are stored.
2. Interoperability between different wallets - where the ownership of DIDs and VCs are prooved.

## 1.3 Rammer

The BSc project runs between `01.feb.2021` and `20.may.2021`.

The BSc project will be developed as an open source project on the DIN-Foundation's Github page here: https://github.com/DIN-Foundation/bsc-ntnu-2020/. 

See https://www.din.foundation/ for more info.


# 2. Omfang

## 2.1 Fagområde

The project is developed within the field of Self-sovereign-identity, aka SSI.

## 2.2 Avgrensing

The project is looking at interoperability of the current SSI-infrastructure. More specifically interoperability between wallets, and interoperability between ledgers.

To understand interoperability a proof-of-concept application will be developed. The proof-of-concept will implement one specific use-case of SSI. 

The proof-of-concept should implement the following parts of an SSI-workflow:
- Issuing a credential to a ledger
- Hold the credential in a wallet
- Proove ownership of a credential to a 3rd party.

A limited set of ledgers and wallets should be investigated:

**TODO List of wallets to investigate**

**TODO List of ledgers to investigate**

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

| Navn        | Role                                               |
|-------------|----------------------------------------------------|
| Jonas       | Developer     (Service Delivery Manager in Kanban) |
| Snorre      | Client        (Service Request Manager in Kanban)  |
| Mariusz     | Domain expert 1                                    |
| Abylay      | Domain expert 2                                    |
| Deepti      | Academic supervisor                                |

## 3.2 Rutiner og regler i gruppa

### Developer rules

1. To keep the project moving at all times, work on the project every day of the week. Minimum 5 minutes, maximum 8 hours.
2. Average 30 hours each week.
3. Respond to NTNU-mail, Discord-messages, Github-comments, Phone-calls, SMS in less than 24 hours.
4. Write meeting-notes for every meeting.
5. Log all hours in Toggl under these 5 categories: [Meeting, Organizing, Writing, Coding, Researching]
6. All documents relevant to the project, should be found in the Github repository, unless it contains sensitive/private information.

# 4. Planlegging, oppfølging og rapporter

## 4.1 Hovedinndeling av prosjektet

The project is loosely driven forward by the Kanban framework. To keep track of progress in a Kanban project the Kanban board is essential. See the board here: https://github.com/DIN-Foundation/bsc-ntnu-2020/projects/1.

The Kanban board has 5 columns:

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

## 4.2 Plan for statusmøter og beslutningspunkter i perioden

We have planned 2 weekly meetings, 1 with supervisor and 1 with client and domain experts.

- Client and Domain experts meeting: Tuesdays @ 12:30
- Academic supervisor meeting: Tuesdays @ 13:00

Participants of these meetings have received invites by email, to make it easy to add the scheduled meetings to their respective calendars.

# 5. Organisering av kvalitetssikring

## 5.1 Dokumentasjon, standardbruk og kildekode
## 5.2 Konfigurasjonsstyring
## 5.3 Risikoanalyse (identifisere, analysere, tiltak, oppfølging)

# 6. Plan for gjennomføring

The project starts in Week 5 (01.feb - 07.feb) and ends in Week 20 (17.may - 23.may). The deadline for delivering the project is 20.may.

### Timespan
- 15 weeks and 3 days
- 108 days
- 78 weekdays
- 624 weekdayhours (8 hours per weekday)

### Milestones Gantt chart

| Milestone            |Week 5|Week 6|Week 7|Week 8|Week 9|Week 10|Week 11|Week 12|Week 13|Week 14|Week 15|Week 16|Week 17|Week 18|Week 19|Week 20|
|----------------------|------|------|------|------|------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|-------|
| Proof of concept app |  +   |  -   |  -   |   -  |  -   |   -   |   -   |   -   |   -   |   -   |   -   |   \|  |       |       |       |       |
| Ledger comparison    |      |      |      |      |      |       |       |       |   +   |   -   |   -   |   -   |   \|   |       |       |       |
| Wallet comparison    |      |      |      |      |      |       |       |       |       |   +   |   -   |   -   |   -   |  \|    |       |       |
| Report               |  +   |  -   |  -   |   -  |  -   |   -   |   -   |   -   |   -   |   -   |   -   |   -   |   -   |  -    |   -   |   \|   |

*Legend: `+` is start of milestone. `-` is ongoing work on a milestone. `|` is the milestone deadline.*


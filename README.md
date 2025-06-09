# BAQ: Bunnybots Automated Queue

An automated robot queuing system for the Bunnybots tournament

## Purpose

- Previously, teams would queue in a physical line with their robots, where after every match, the top 8 teams would draw to form teams of 3 with 2 teams being passed to the next round.
- This was cumbersome and meant that by chance, some teams would play each other more often, resulting in a scewed score-to-skill distribution
- BAQ aims to be a streamlined digital queue, to replace the physical one

### Goals

- Be easily accessible from a phone
- Have an authorization system, to ensure only appointed members from each team can remove and add themselves
- Notify users as to when they should be on deck in a timely manner
- Reorder the queue ad hoc to ensure that teams play each other in a roughly even distribution (should yield to the above goal)

## Notes

- This app is very much WIP and experimental, whether it will be used at Bunnybots 2025 or not is TBD

## Developing

```bash
bun i

bun run dev
```

## Building

To create a production version of your app:

```bash
bun run build
```

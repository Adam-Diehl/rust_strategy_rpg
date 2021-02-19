# Rust Strategy RPG (working title)
This ~~is~~ will be a strategic squad-based role playing game (RPG) where players take on the role of a commander recruiting units, assigning them to squads, and then deploying those squads in battles that play out automatically based on the assigned units' abilities and behaviors. Right now, the game's engine is being developed. This repository will include the files for the game engine, and associated test data. Any actual game data (essentially, an implementation using the engine) will be distributed through other channels.

The game is written in pure, safe Rust.

### Where it's at
Right now I'm still working on basic engine features, so while the game is technically "playable" there isn't much content to actually play with. Once the engine hits a stable state, I plan to backfill with content.

What I've added:
- Auras: pre-battle buffs to allies. Currently restricted to health and power.
  - 2021/02/11: auras can now apply to allies too (so a hero can buff the health/power of their whole squad).
- Abilities (WIP): active effects triggered by certain events during combat (currently, just attacks).
  - 2021/02/16: abilities triggered when attacking now work for self, allies, and enemies. These abilities are the same stat changes as auras. I've confirmed they work manually but haven't added unit tests for them yet.

### How to Play<sup>\*</sup>

<sup>\*</sup>Again, gameplay is *incredibly* barebones at the moment and really only available for test purposes. When you run the compiled binaries (not included...some compilation required) you should see the following message:

```
Welcome to <Rust Strategy RPG>! [Working title]
You must defeat a squad of evil-doers consisting of the following characters:
--------------------
Name: The Dragon
Health: 163800
Power: 6000 | Crit. Chance: 10%
Speed: -2
Attacks all and drains enemy power by 20%.
Grants allies 40% health.
--------------------

--------------------
Name: The Evil Tank
Health: 127400
Power: 8400 | Crit. Chance: 5%
Speed: -1
Attacks a single enemy and heals 35%.
Grants allies 30% health.
--------------------

--------------------
Name: The Evil Swordsman
Health: 218400
Power: 9600 | Crit. Chance: 5%
Speed: -1
Attacks a single enemy.
Grants self 60% health.
--------------------

--------------------
Name: The Evil Mage
Health: 81900
Power: 18000 | Crit. Chance: 10%
Speed: -3
Attacks all, slows by 1, and power gains allies 10%.
Grants allies 20% power.
--------------------

You can choose from the following characters:
 - GoodArcherRogue
 - GoodMeleeRogue
 - GoodSwordsman
 - GoodTank

```

You will be asked to fill the four slots in your squad. There are four positions: front-left, front-right, back-left, and back-right. The front row will tend to get attacked first (some characters, like the mage, have AOE, and others like the "GoodMeleeRogue" can flank), so tanks should go here. You can assemble a squad of 4 characters from the following test units (you can actually use 'evil' units, it's just not prompted in the UI):
- Dragon
- EvilSwordsman
- GoodArcherRogue
- GoodSwordsman
- EvilMage
- EvilTank
- GoodMeleeRogue
- GoodTank

Their statistics are in the `data/characters` folder. Note that the game is displaying "effective" statistics, *after* auras are applied. Once the game is closer to completion I'll add more detail here, but basically, the game works in the following way:
- All heroes have health and power statistics.
- When they make an attack, the power stat is subtracted off from the target's health stat, plus or minus armor & dodge ("dr" and "dt"), critical hits, etc.
- All heroes have a preset targeting behavior. Most heroes target the front-liners in the enemy squad (put your tanks here!), but rogues and other slippery creatures can flank and attack the back row. Some characters, like the `dragon` in the test character set, attacks every character with its fiery breath.  
- You "win" when the entire enemy squad is dead. (Since battles are automated, this is kinda boring, I know. The point of the game will be on acquiring, assembling, and deploying the right squad for the right challenges, but that comes later.)

Your AI opponent controls the following squad:
- Dragon (FL)
- EvilTank (FR)
- EvilSwordsman (BL)
- EvilMage (BR)

Battles play out automatically once you choose your squad based on the character's properties and behaviors.

For instance, suppose that you choose the following squad:

```
Welcome to <Rust Strategy RPG>! [Working title]
Choose character for the front-left position:
GoodTank
Choose character for the front-right position:
GoodSwordsman
Choose character for the back-left position:
GoodMeleeRogue
Choose character for the back-right position:
GoodArcherRogue
```

Then this is one of the possible battle outcomes (playing the game in a terminal provides bold and color coded output):

<details>
<summary>Sample Game Output</summary>
<br>

```
# --- ROUND 1 --- #
The Good Archer is attacking The Dragon! The Dragon took 20000 points of damage (78% HP remaining).
The Good Archer is attacking The Evil Swordsman! The Evil Swordsman took 20000 points of damage (73% HP remaining).
The Good Swordsman is attacking The Dragon! The Dragon took 12000 points of damage (64% HP remaining).
The Good Swordsman is attacking The Evil Tank! The Evil Tank took 12000 points of damage (83% HP remaining).
The Evil Swordsman is attacking The Good Tank! The Good Tank took 8000 points of damage (87% HP remaining).
The Evil Tank is attacking The Good Tank! Critical hit! The Good Tank took 14000 points of damage (63% HP remaining).
The Good Rogue is attacking The Evil Mage! The Evil Mage took 18500 points of damage (59% HP remaining).
The Good Tank is attacking The Dragon! The Dragon took 7000 points of damage (57% HP remaining).
The Dragon is attacking The Good Tank! Critical hit! The Good Tank took 10000 points of damage (47% HP remaining).
The Dragon is attacking The Good Swordsman! The Good Swordsman took 5000 points of damage (88% HP remaining).
The Dragon is attacking The Good Rogue! The Good Rogue took 5000 points of damage (92% HP remaining).
The Dragon is attacking The Good Archer! The Good Archer took 5000 points of damage (80% HP remaining).
The Evil Mage is attacking The Good Tank! The Good Tank took 12000 points of damage (27% HP remaining).
The Evil Mage is attacking The Good Swordsman! The Good Swordsman took 12000 points of damage (57% HP remaining).
The Evil Mage is attacking The Good Rogue! Critical hit! The Good Rogue took 24000 points of damage (55% HP remaining).
The Evil Mage is attacking The Good Archer! The Good Archer took 12000 points of damage (32% HP remaining).

# --- ROUND 2 --- #
The Good Archer is attacking The Dragon! Critical hit! The Dragon took 40000 points of damage (12% HP remaining).
The Good Archer is attacking The Evil Swordsman! The Evil Swordsman took 20000 points of damage (47% HP remaining).
The Good Swordsman is attacking The Dragon! The Dragon took 12000 points of damage (0% HP remaining).
The Dragon died!
The Good Swordsman is attacking The Evil Tank! The Evil Tank took 12000 points of damage (66% HP remaining).
The Evil Swordsman is attacking The Good Tank! The Good Tank took 8000 points of damage (13% HP remaining).
The Evil Tank is attacking The Good Tank! The Good Tank took 7000 points of damage (2% HP remaining).
The Good Rogue is attacking The Evil Mage! The Evil Mage took 18500 points of damage (18% HP remaining).
The Good Tank is attacking The Evil Tank! The Evil Tank took 7000 points of damage (56% HP remaining).
The Evil Mage is attacking The Good Tank! The Good Tank took 12000 points of damage (0% HP remaining).
The Good Tank died!
The Evil Mage is attacking The Good Swordsman! Critical hit! The Good Swordsman took 24000 points of damage (0% HP remaining).
The Good Swordsman died!
The Evil Mage is attacking The Good Rogue! The Good Rogue took 12000 points of damage (37% HP remaining).
The Evil Mage is attacking The Good Archer! The Good Archer took 12000 points of damage (0% HP remaining).
The Good Archer died!

# --- ROUND 3 --- #
The Evil Swordsman is attacking The Good Rogue! The Good Rogue took 8000 points of damage (25% HP remaining).
The Evil Tank is attacking The Good Rogue! The Good Rogue took 7000 points of damage (14% HP remaining).
The Good Rogue is attacking The Evil Mage! Critical hit! The Evil Mage took 37000 points of damage (0% HP remaining).
The Evil Mage died!

# --- ROUND 4 --- #
The Evil Swordsman is attacking The Good Rogue! The Good Rogue took 8000 points of damage (2% HP remaining).
The Evil Tank is attacking The Good Rogue! Critical hit! The Good Rogue took 14000 points of damage (0% HP remaining).
The Good Rogue died!
```

</details>

### Planned Game Modes
Campaign mode:
- Make tough choices between units you can recruit and squads you can assign to objectives.

Challenge mode:
- Fight waves of increasingly powerful enemies.

Multiplayer mode:
- As of now, the plans for multiplayer are limited to sharing/importing squad configurations from other players (so not a live service).

# Code Base

### Source Code
The following files are contained in the `src` folder.
- `character.rs`: defines the character struct and implementation details (`impl`)
- `combat.rs`: manages the core loop of combat
- `configs.rs`: sets global defaults (as `const` parameters)
- `input.rs`: manages user input
- `main.rs`: runs the program
- `modifiers.rs`: handles auras (passive modifier applied before combat) and abilities (active modifier applied during combat)
- `squad.rs`: manages the creation of squads from collections of characters
- `targeting.rs`: given a character's targeting preferences and a list of valid (i.e. alive) targets, determines the right characters to attack

### To do
- ~~Allow multiple attacks and attacks of multiple targets~~ (Added 2021-01-16)
- Add auras and abilities (adding auras is my current focus)
- Add tests to squad functions
- Add mechanics to levels/leveling up

### Build & Test Status

Current game build: **playable**

Current test status: **all passing**

<details>
<summary>Test details</summary>
<br>

```
running 25 tests
test character::tests::test_new ... ok
test character::tests::test_is_dead ... ok
test character::tests::test_take_damage_dr_only ... ok
test character::tests::test_take_damage_dt_and_dr ... ok
test character::tests::test_take_damage_dt_only ... ok
test character::tests::test_take_damage_no_armor ... ok
test combat::tests::test_calculate_initiave ... ok
test modifiers::tests::test_aura_change_crit_chance ... ok
test modifiers::tests::test_aura_change_health ... ok
test modifiers::tests::test_aura_change_power ... ok
test modifiers::tests::test_aura_change_speed ... ok
test modifiers::tests::test_aura_convert_and_add_i32 ... ok
test modifiers::tests::test_aura_convert_and_multiply_f64 ... ok
test modifiers::tests::test_aura_convert_and_multiply_i32 ... ok
test targeting::tests::test_attack_to_coordinates_all ... ok
test targeting::tests::test_attack_to_coordinates_left_column ... ok
test targeting::tests::test_attack_to_coordinates_right_column ... ok
test targeting::tests::test_attack_to_coordinates_row_back ... ok
test targeting::tests::test_attack_to_coordinates_row_back_flanker ... ok
test targeting::tests::test_attack_to_coordinates_row_back_partial ... ok
test targeting::tests::test_attack_to_coordinates_row_front ... ok
test targeting::tests::test_attack_to_coordinates_row_front_partial ... ok
test targeting::tests::test_attack_to_coordinates_single_back ... ok
test targeting::tests::test_attack_to_coordinates_single_flanker ... ok
test targeting::tests::test_attack_to_coordinates_single_front ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

</details>

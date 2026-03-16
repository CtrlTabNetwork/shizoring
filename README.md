# Shizoring

Elden Ring (dlc compatible) savefile "modder" with built-in utilities

## Why does this tool exist?
This tool is made for people (like me) who really like looting without thinking twice and dealing with the consequences later.

> _Nothing wrong about having 50 copies of a footsoldier's helmet..._

The only functionality (for now) of this tool is **deduplication** of armor, talismans and weapons (up to 2, because of dual wielding)  

## I have the attention span of a goldfish so just tell me how to use it
Sure bud, but first of all **you need to backup your savefile. I don't want to be the one responsible for you losing a thousand hours on your game.**

---

How to backup save file? https://eldenring.wiki.fextralife.com/Save+Data+Backup+Management

After you did that, if you run windows just download the executable file in the right sidebar under the latest release.

Drag and drop the executable in a folder of your liking alongside your savefile (usually ".sl2" or ".co2", both are compatible), then open a terminal and write:
```
shizoring <filename_with_extension>
```
Now you will be prompted to select the character (identified by the name) and the type of deduplication.

```
Select character:
> Ranni del Napoli
  Ran
  Ranni Del Molise

Select items to deduplicate:
> [x] Armors
  [x] Weapons (down to 2 copies)
  [x] Talismans (except Sacrificial Twig)
```

You can select multiple items to deduplicate by using spacebar, then confirm by pressing enter, after that there the program will tell you each item that's gonna be removed (and which ones will be skipped, mainly sacrificial twigs)

```
Select character: Ranni del Napoli
Select items to deduplicate: Armors, Weapons (down to 2 copies), Talismans (except Sacrificial Twig)
Duplicate item: Sacrificial Twig (Talisman) (ID: 6070) [skipped]
Duplicate item: Sacrificial Twig (Talisman) (ID: 6070) [skipped]
Duplicate item: Viridian Amber Medallion +1 (Talisman) (ID: 1021)
Duplicate item: Carian Filigreed Crest (Talisman) (ID: 6020)
Duplicate item: Stargazer Heirloom (Talisman) (ID: 1080)
Duplicate item: Boltdrake Talisman (Talisman) (ID: 4030)
Duplicate item: Verdigris Helm (Armor) (ID: 5010000)
```

After the end of the process, a new file will appear in your folder named "deduped", this file is the new modded savefile. You can put it in the game the same way you extracted it (renaming the original like "OLD" and renaming "deduped" in "ER0000", and keeping the extension).

---

## I'm a rust user so i know how to compile it but i still want you to tell me how to do it 👉👈
```
cargo build --release
```
I tried my best not to be the "it works on my machine" guy but if it breaks know that it works on my machine

---

# WILL THIS BAN ME?
Maybe, maybe not, just be safe and use it in an offline game, or just download seamless co-op mod to be safe 100%

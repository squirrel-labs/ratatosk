# DiscoBot
A sipmle Discord Bot, interfacing to  the DSA "Heldensoftware", for a minimalistic and low labor aproach to digitalize p&amp;p rpg's


This software takes helden.zip.hld files and loads them into the Bot.
On enabled Servers, it provides a text based open source customizable text interface for characterspecific actions.
e.g. !attack Bogen +4 to shoot a bow.
The Bot recognizes the Sender and asosiates one of the loaded Chars to get the correct values.
At the time, also an basic dice emulator is implemented. 
More features can be easily added (namegenerator...) 

# Usage
there are four main character specific action groups:
!talent {name} [t,z,zauber...]
!angriff {weapon} [a,attacke,attacke_mit...]
!parade {weapon} [p,P,parade_mit]
!fernkampf {weapon} {erschwernis} [schuss,f,F,schieße_mit]

-mainly the talent group has enhanced type corrrection
-"erschwernis" is optional

Roll command
!roll {dice string e.g. "3d20"} [r,R,Würfle]

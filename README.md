# rs-music

Create sweet tunes with rust

## Usage 
Currently this project is exists in a limbo between library and binary. The easiest way to try it out is to fork this repo and `include_str!` your own file.

## Ascii Sheet Music
There are many attempts out there to represent sheet music in some kind of readable ascii but since none of them were to my liking i decided to make my own.

### Key
A good place to start is to define your key
```
Key:
K: [Pitch][Accidental]?(M? | m)
```
Currently the 30 major, `M`, and minor, `m`, keys are supported.
If no key is specified C major is assumed.

### BPM
BPM sets the length of notes by defining how many beats (1/4th notes) are in a minute
```
BPM:
BPM: [u32]
```
If no BPM is specified 120 is assumed.

### Signature
Signature defines the length of each bar
```
Signature:
[u32]/[u32]
```
If no Signature is specified 4/4 is assumed.

### Bar lines
Optional bar lines can be used to force the program to validate that each bar is the proper length.

#### Note
Even if bar lines are not used, bar length will be validated when Key, BPM or Signature is set to confirm that these are only changed between two bars.

### Notes and Rests

#### Notes
To play music you need notes, now you have them
```
Note:
[Pitch][i32]?[Accidental]?[Duration]?
```

#### Rests
If silence is what you need you should rest
```
Rest:
R[Duration]?
```

#### Pitch
All notes have a pitch
```
Pitch:
(A|B|C|D|E|F|G)
```

#### Accidental
Change the pitch by half steps
```
Accidental:
(#* | b* | n)
```
Accidental will also be iferred by the key

#### Duration
Durations define the length of a note in fractions or sets thereof
```
Duration:
/([u32].*~?)+
```
The given fractional part(s) must be powers of 2

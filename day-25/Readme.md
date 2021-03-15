# Day 25

[Advent Of Code Day 25](https://adventofcode.com/2020/day/25).

## Part 1

Check In Desk

* room key is a small RFID card for room on the 25th floor, card does not open door
* room card sends a command
* cryptographic handshake to communicate between card and door
* operation transforms a **subject number**
* start with value `1`
  * set value to itself multiplied by **subject number**
  * set value of remainder after deviding the value by `20201227`
* card uses a specifc **secret loop size**
* door uses a different **secret loop size**

Cryptographic handshake works as follows:

* **card** transforms the subject number of `7` according to its loop size, the **card public key**
* **door** transforms the subject number of `7` according to its loop size, the **door public key**
* card and door use wireless RFID signal to transmit the two public keys to the other device.
  * now **card** has the **door** public key and vice versa
  * public keys are known (intercepted)
* **card** transforms subject number of **door public key** according to **card** loop size, the **encryption key**
* **door** transforms subject number of **card public key** according to **door** loop size, the **encryption key**
* both encryption keys are the same!

Example

* card public key: `5764801`
* card loop size is `8`, because tranformining initial number `7` with a loop size of `8` results in `5764801`
* door public key: `17807724`
* door loop size is `11`, because transforming initial number `7` with a loop size of `1` results in `17807724`
* use either loop size with public key to calculdate encryption key
  * transform **door** public key `17807724` by loop size `8` (**card** loop size) produces `14897079`

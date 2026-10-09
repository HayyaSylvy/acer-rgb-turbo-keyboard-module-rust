### Acer RGB keyboard backlight and Turbo mode Linux kernel module (Acer Predator, Acer Helios, Acer Nitro)
![](keyboard.webp)

[![GitHub repo Good Issues for newbies](https://img.shields.io/github/issues/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module/
    
Inspired by [faustus(for asus)](https://github.com/hackbnw/faustus) and by [faustus(for asus)](https://github.com/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module), this project extends the current Acer-WMI Linux kernel module to support Acer gaming functions. Written in Rust now :3


> **Warning**
> ## Use at your own risk! Acer was not involved in developing this driver, and everything is developed by reverse engineering the official Predator Sense app. This driver interacts with low-level WMI methods that haven't been tested on all series and an additional Rust rewrite.

**Will this work on my laptop?**

Compatibility table:


| Product name |                                           Turbo Mode (Implemented)                                           |                                             Turbo Mode (Tested)                                             | RGB (Implemented) | RGB (Tested) |
|--------------|:------------------------------------------------------------------------------------------------------------:|:-----------------------------------------------------------------------------------------------------------:|:-----------------:|:------------:|
| AN515-45     |                                                      -                                                       |                                                      -                                                      |        Yes        |     Yes      |
| AN515-55     |                                                      -                                                       |                                                      -                                                      |        Yes        |     Yes      |
| AN515-56     |                                                      -                                                       |                                                      -                                                      |        Yes        |     Yes      |
| AN515-57     |                                                      -                                                       |                                                      -                                                      |        Yes        |     Yes      |
| AN515-58     |                                                      -                                                       |                                                      -                                                      |        Yes        |     Yes      |
| AN517-41     |                                                      -                                                       |                                                      -                                                      |        Yes        |     Yes      |
| PH315-52     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PH315-53     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PH315-54     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PH315-55     |                                                     Yes                                                      |    |        Yes        |      No      |
| PH317-53     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PH317-54     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PH517-51     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PH517-52     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PH517-61     | [Partial#94](https://github.com/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module/issues/94)  | [Partial#94](https://github.com/JafarAkhondali/acer-predator-turbo-and-rgb-keyboard-linux-module/issues/94) |        Yes        |     Yes      |
| PH717-71     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PH717-72     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PHN18-71     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PT314-51     |                                                      No                                                      |                                                     No                                                      |        Yes        |     Yes      |
| PT315-51     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PT314-52s    |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |      No      |
| PT315-52     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PT316-51     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PT316-51s     |                                                    Yes                                                      |                                                     Yes                                                     |        Yes        |     No       |
| PT515-51     |                                                     Yes                                                      |                                                     Yes                                                     |        Yes        |     Yes      |
| PT515-52     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |
| PT516-52s    |                                                     Yes                                                      |                                                     No                                                      |        Yes        |     Yes      |
| PT917-71     |                                                     Yes                                                      |                                                     No                                                      |        Yes        |      No      |


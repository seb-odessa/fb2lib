# fb2lib

```
$ ./target/debug/fb2lib data/arch.zip ls
book1.fb2            241143       562015
book2.fb2            235547       783762
book4.fb2             85787       160987
book3.fb2            463975      1118067
book5.fb2            631985      1505856
```

```
$ ./target/debug/fb2lib data/arch.zip info book1.fb2
<?xml version="1.0" encoding="Windows-1251"?>
<FictionBook xmlns="http://www.gribuser.ru/xml/fictionbook/2.0" xmlns:l="http://www.w3.org/1999/xlink">
 <description>
  <title-info>
   <genre>sf</genre>
   <genre>sf_history</genre>
   <author>
    <first-name>Константин</first-name>
    <middle-name>Георгиевич</middle-name>
    <last-name>Калбанов</last-name>
    <home-page>http://samlib.ru/k/kalbazow_k_g/</home-page>
    <email>mahoni928@yandex.ru</email>
   </author>
   <book-title>Робинзоны</book-title>
   <annotation>
    <p>Каменный век и немного прогрессорства.</p>
   </annotation>
   <date></date>
   <coverpage>
    <image l:href="#ac6591c8dd81e13edc3209e7273fd309.jpg"/></coverpage>
   <lang>ru</lang>
   <sequence name="Робинзоны" number="1"/>
  </title-info>
  <document-info>
   <author>
    <nickname>54321876875</nickname>
   </author>
   <program-used>FictionBook Editor Release 2.6</program-used>
   <date value="2012-09-06">06-09-2012</date>
   <src-url>http://samlib.ru/k/kalbazow_k_g/kot12.shtml</src-url>
   <src-ocr>СИ</src-ocr>
   <id>A555657E-48F3-4B79-8A46-0F9D18908A3C</id>
   <version>1.2</version>
   <history>
    <p>v. 1.2 — создание FB2 (54321876875)</p>
   </history>
  </document-info>
 </description>
</FictionBook>
```


# patch-xml

[![Actions Status](https://github.com/VilNeo/patch-xml/workflows/Test/badge.svg)](https://github.com/VilNeo/patch-xml/actions)
[![Crates.io](https://img.shields.io/crates/v/patch-xml.svg)](https://crates.io/crates/patch-xml)
[![Docs](https://docs.rs/patch-xml/badge.svg)](https://docs.rs/crate/patch-xml/)

***patch-xml* is a tool and library that reads and patches XML files.**

##Usage

A given XML file is patched by applying a yaml-file on it that contains the corresponding modification rules. This can be achieved via the command line or by using the library in other crates.

###Command line tool
The tool requires the original XML file and the patching yaml-file and writes the result into a final XML file. Therefore it requires three arguments:
1. The path to the original xml file
2. The path to the patch file in yaml format
3. The path to the final result file that will be written (on success)

###Library
*coming soon*

##Patching syntax
A given XML tree can be patched with a yaml-file that provodes a number of modification rules.
In the following these modification rules with their individual command patterns are described:

###Special characters
There exist two different special characters:
1. **The `$` character** is the prefix of command patterns that allow powerful modifications.
   It can be escaped with `$$`.
2. **The `[` character** introduces a reference (e.g. `[..:1]`) to a caption of a regular expression. It can be escaped with `\[`.
3. **The `.` character** is the prefix for attributes.

###Overwrite
An **existing entry** can be overwritten directly. If there is no existing entry, a new entry is created.
#####Input (XML):
  ```xml
  <parent>
   <simple>Foo</simple>
   <complex>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex>
</parent>
  ```
#####Patch (YAML):
  ```yaml
  parent:
     simple: Updated
     complex:
        cval2: Baaaaaar
        cval3: Baz
  ```
#####Result (XML):
  ```xml
  <parent>
   <simple>Updated</simple>
   <complex>
      <cval1>Foo</cval1>
      <cval2>Baaaaaar</cval2>
      <cval3>Baz</cval3>
   </complex>
</parent>
  ```
###Create new enttries
An arbitrary amount of **new entries** can be added with the command `$add`. This can be combined with a modification or removing (`~`) pattern.
#####Input (XML):
  ```xml
  <parent>
   <simple>Foo</simple>
   <complex>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex>
</parent>
  ```
#####Patch (YAML):
  ```yaml
  parent:
     simple: Overwritten
     $add:
        - simple:
             Bar
        - simple:
             Baz
        - complex:
             cval2: Baaaaaar
             cval3: Baz
  ```
#####Result (XML):
```xml
<parent>
   <simple>Overwritten</simple>
   <simple>Bar</simple>
   <simple>Baz</simple>
   <complex>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex>
   <complex>
      <cval2>Baaaaaar</cval2>
      <cval3>Baz</cval3>
   </complex>
</parent>
  ```
###XML Attributes
XML **attributes** can be accessed with a `.`-prefix.
#####Input:
  ```xml
  <parent att="Foo">
</parent>
  ```
#####Patch:
  ```yaml
  parent:
     .att: Updated
  ```
#####Result:
  ```xml
  <parent att="Updated">
</parent>
  ```
###Removing entries
Entries can be **removed** by setting them to `null` or `~`.
#####Input:
  ```xml
  <parent>
   <simple>Foo</simple>
   <complex1>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex1>
   <complex2>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex2>
</parent>
  ```
#####Patch:
  ```yaml
  parent:
     simple: ~
     complex1: ~
  ```
#####Result:
    ```xml
    <parent>
      <complex2>
        <cval1>Foo</cval1>
        <cval2>Bar</cval2>
      </complex2>
    </parent>
    ```
###Replacing entries
**Simple entries do not need to be replaced explicitely** since a modification is always a replace there.

**Complex** entries can be replaced by **removing** the old entry **and adding** a new one.

#####Input (XML):
  ```xml
  <parent>
   <simple>Foo</simple>
   <complex>
      <cval1>Foo</cval1>
   </complex>
</parent>
  ```
#####Patch (YAML):
  ```yaml
  parent:
     simple: Overwritten
     complex: ~
     $add:
        - complex:
             cval2: Baaaaaar
             cval3: Baz
  ```
#####Result (XML):
```xml
<parent>
   <simple>Overwritten</simple>
   <complex>
      <cval2>Baaaaaar</cval2>
      <cval3>Baz</cval3>
   </complex>
</parent>
  ```

###Paths
Within a patch there can be defined paths (i.e. in ``$move``, ``$copy``, ``$import`` and when using captures from regular expressions). These paths have unix like syntax, which means:
- `/` is the separator
- `..` is parent level
- `.` is current level
- absolute paths start with `/`

While the usual commands (modify, remove, add,...) act on the result, the paths are applied on the original input.

###Regular expressions
Keys are interpreted as **regular expressions**.
They support captures which can be used from values to access the matched string or even subparts of it.
A capture can be defined by putting parts of a regular expression into braces (e.g. `Fo(.?)` ).
In values, the captures can be accessed by an index. Index 0 is always the fully matched string. The nth index is the nth capture.
E.g. when applying ``Hello World`` on the regular expression ``.*(Wo)(rld)`` we get the captures:
- at index 0: ``Hello World``
- at index 1: ``Wo``
- at index 2: ``rld``

The access of a value to these captures is achieved with the pattern ``[<path_to_element>:<capture_index>]``.

#####Input:
  ```xml
  <parent>
   <simple1>Foo</simple1>
   <simple2>Bar</simple2>
   <simple3>Baz</simple3>
   <complex1>
      <cval1>Foo</cval1>
   </complex1>
   <complex2>
      <cval1>Foo</cval1>
   </complex2>
   <complex3>
      <cval1>Foo</cval1>
   </complex3>
</parent>
  ```
#####Patch:
  ```yaml
  parent:
     simple(.?): SimpleUpdated[.:1]
     complex(.?):
        cval1: ComplexUpdated[..:1]
  ```
#####Result:
  ```xml
  <parent>
   <simple1>SimpleUpdated1</simple1>
   <simple2>SimpleUpdated2</simple2>
   <simple3>SimpleUpdated3</simple3>
   <complex1>
      <cval1>ComplexUpdated1</cval1>
   </complex1>
   <complex2>
      <cval1>ComplexUpdated2</cval1>
   </complex2>
   <complex3>
      <cval1>ComplexUpdated3</cval1>
   </complex3>
</parent>
  ```
###Moving and copying
Entries can be **moved** and **copied** by setting `$move` or `$copy` to the target path.
#####Input:
```xml
  <parent>
   <simple>Foo</simple>
   <complex>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex>
</parent>
 ```

#####Patch:
```yaml
  parent:
     simple:
        $move: simple-moved
     complex:
        cval1:
           $move: ../upmoved-value
        cval2:
           $copy: ../upcopied-value
```

#####Result:
```xml
  <parent>
   <simple-moved>Foo</simple-moved>
   <complex>
      <cval2>Bar</cval2>
   </complex>
   <upmoved-value>Foo</upmoved-value>
   <upcopied-value>Bar</upcopied-value>
</parent>
```
###Self referencing
When using `$move`, `$copy` or `$filter` it is possible to use `$self` to set the value of the surrounding **element itself**. If`$self` is used, it must be used exclusively. When copying, changes are applied to the copy only.
#####Input:
  ```xml
  <parent>
   <simple>Foo</simple>
   <complex>
      <cval1>Foo</cval1>
      <cval2>Bar</cval2>
   </complex>
</parent>
 ```
#####Patch:

```yaml
  parent:
     simple:
        $move: simple-moved
        $self: Moved
     complex:
        $move: complex-moved
        $self:
           cval1: Moved
           cval2:
              $copy: ../upcopied-value
              $self: Changed
```

#####Result:
```xml
  <parent>
   <simple-moved>Moved</simple-moved>
   <complex-moved>
      <cval1>Moved</cval1>
      <cval2>Bar</cval2>
   </complex-moved>
   <upcopied-value>Changed</upcopied-value>
</parent>
```
###Filtering
Multiple elements with the same name can be **filtered** by using a `$filter` entry. The content of the `$filter` element consists of a map where both, the keys and values are regular expressions. These must be fulfilled by the surrounding element. Their captures are accessible to the other elements as well.
```xml
  <parent>
   <simple>Foo</simple>
   <simple>Bar</simple>
   <simple>Baz</simple>
   <complex>
      <cval1>Foo</cval1>
   </complex>
   <complex>
      <cval1>Bar</cval1>
   </complex>
   <complex>
      <cval1>Baz</cval1>
   </complex>
</parent>
```
#####Patch:
```yaml
  parent:
     simple:
        $filter:
           $self: Ba.*
        $self: Updated [$filter/$self/:0]
     complex:
        $filter:
           cval1: Bar
        cval1: Baaaar
```
#####Result:
```xml
  <parent>
   <simple>Foo</simple>
   <simple>Updated Bar</simple>
   <simple>Updated Baz</simple>
   <complex>
      <cval1>Foo</cval1>
   </complex>
   <complex>
      <cval1>Baaar</cval1>
   </complex>
   <complex>
      <cval1>Baz</cval1>
   </complex>
</parent>
```
###Importing
Each element (even within `$filter` elements) can be **imported** from another yaml file by using the key `$import`.
```xml
  <parent>
   <simple>Foo</simple>
   <simple>Bar</simple>
   <simple>Baz</simple>
   <complex>
      <cval1>Foo</cval1>
   </complex>
   <complex>
      <cval1>Bar</cval1>
   </complex>
   <complex>
      <cval1>Baz</cval1>
   </complex>
</parent>
```
#####lib.yaml:
```yaml
  simple:
     $filter:
        $self: Ba.*
     $self: Updated [$filter/$self/$value:0]
  bar-filter:
     cval1: Bar
```
#####Patch:
```yaml
  parent:
     simple:
        $import: lib.yaml:/simple
     complex:
        $filter:
           $import: lib.yaml:/bar-filter
        cval1: Baaaar
  ```
#####Result:
```xml
  <parent>
   <simple>Foo</simple>
   <simple>Updated Bar</simple>
   <simple>Updated Baz</simple>
   <complex>
      <cval1>Foo</cval1>
   </complex>
   <complex>
      <cval1>Baaar</cval1>
   </complex>
   <complex>
      <cval1>Baz</cval1>
   </complex>
</parent>
```

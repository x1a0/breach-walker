Breach walker
=============

A pathfinder for breach world in BatMud.


How to use
----------

```shell
breach-walker <from> <to>
```

Both `<from>` and `<to>` take coordinates in the form of `x:y`.


TinyFugue example
-----------------

This macro reads player current location from PROMPT so you need to have
`<coord>` in Bat PROMPT and update the regex to match your PROMPT.

```tf
;; /breach_go x:y
;; If x:y is not provided it will go back to where you call this last time, as
;; a return trip.
/def -i breach_go=\
  /if ({#} < 1) \
    /let _dest=%{bgo_last_from}%;\
  /else \
    /let _dest=%{*}%;\
  /endif%;\
  /def -F -1 -p99999 -i -mregexp -h"PROMPT \\[\\d+\\]\\[\\$$:\\d+/\\d+\\]\\[\\d+kg\\]\\[\\w+\\|(\\d+),(\\d+)\\]\\[\\w+\\] >" _bgo_prompt_hook=\
    /test breach_go_do("%%{P1}:%%{P2}", "%{_dest}")%;\
  /send%;

/def breach_go_do=\
  /set bgo_last_from=%{1}%;\
  /quote -dsend !breach-walker %{1} %{2}%;

;;; }}}
```

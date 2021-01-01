# Example commands

## Example fact commands

1. Starting off with the most basic example, let's add a new isolated fact entry for last night's sleep:

    ```
    dlog slept 7.5 hr
    ```

This will produce the following:
    - A new abstract fact _"slept"_ with assumed temporal units (implied by "hr"), with default hourly denominations.
    - A new fact entry for "slept" with value 7.5 and the temporal units "Hour".

2. Let's add another fact, this time about a dream we might have had last night:

    ```
    dlog dreamt -a "scary"
    ```

By default, if no value is provided for a fact entry, the fact value will default to a boolean value of "true", such that commands such as `dlog 'brushed teeth'` can be given with the appropriate value without having explicitly give a value of "true" or "yes" for every entry (although that is allowed).

Of note here is the `-a` flag passed in the command -- this creates the specified attribute in your log, and creates an association between the fact entry and the attribute (in this case, the attribute `scary`). Note that attributes given in this way do not apply to all fact entries for `dreamt`, only the ones where we have explicitly passed this argument. If we wish to link an attribute to a fact permanently (establishing a _quality_ of the fact -- more on that [later](../usage_advanced.md)) we should instead pass the flag `-A`.

> **Note**: As a general rule, all links intended to be made for an entry of an object, as opposed to the object in the abstract, are passed with a lowercase short flag -- `-r` to link a record to a fact entry, `-i` to link an item to a fact entry, etc. versus `-R` to link all facts of a type to a given record, `-I` to do the same with an item, etc.

3. We now have two abstract facts, "dreamt" and "slept". Let's

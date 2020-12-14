# dlog
----

## overview
- This will eventually be an application which will allow for highly dynamic and programmatic note- and log-keeping from the terminal. It aims to ultimately be highly intuitive to use at its most base-level, while allowing for a high degree of configurability and flexibility for more advanced use cases.

## notes
- structure of data folder:
    - /data
        - /record1
            - /subrecord
            - items.json
            - item1.csv
            - record1.csv
        - /record2
            - items.json
            - item1.csv
            - record2.csv
        - /inbox
            - items.json
            - item1.csv
            - item2.csv
            - inbox.csv


- **Alternate formulation 1**:
    - Each fact corresponds to one .csv:
        _fact name.csv_
        **Value**    **Units**    **Date**     **Attributes**
         ---      --       --          --       --     --
    - Each item represents an abstract higher order association, without a target
    - So inputting `dlog sleep 5hr -i health` would create the `health` item (if not already created) with an implicit and abstract association with the "sleep" fact as such.
    - An example of an Item would be something like "Food" -- where each fact associated with the "Food" item would be something like `hotdog`, `500cals` which on its own (without context) could be ambiguous as to its purpose or even what it intends to log.
    - A record, instead of being a hierarchically higher kind of Item, simply corresponds to logical groupings the user wishes to make explicitly. A `life` record might track items like `hygiene` (with facts `brushed teeth`, `showered`, `shaved`, etc.), `sleep` (with facts like `duration`, `quality`, `dreamed`, etc.).
        - Based on these groupings, the user can define attributes like `priority`, `notify`, etc. which will serve as the basis for _actions_ the user can define (automations, notifications, etc.)

## todo
- [ ] Make top level args (index 1 and 2) be free values corresponding to fact name and value, respectively
- [ ] Add unit of measure

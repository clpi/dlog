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


### Overview
- Each fact corresponds to one .csv:
    _fact name.csv_
    **Value**    **Units**    **Date**     **Attributes**
     ---      --       --          --       --     --
- Each item represents an abstract higher order association, without a target
- So inputting `dlog sleep 5hr -i health` would create the `health` item (if not already created) with an implicit and abstract association with the "sleep" fact as such.
- An example of an Item would be something like "Food" -- where each fact associated with the "Food" item would be something like `hotdog`, `500cals` which on its own (without context) could be ambiguous as to its purpose or even what it intends to log.
- A record, instead of being a hierarchically higher kind of Item, simply corresponds to logical groupings the user wishes to make explicitly. A `life` record might track items like `hygiene` (with facts `brushed teeth`, `showered`, `shaved`, etc.), `sleep` (with facts like `duration`, `quality`, `dreamed`, etc.).
    - Based on these groupings, the user can define attributes like `priority`, `notify`, etc. which will serve as the basis for _actions_ the user can define (automations, notifications, etc.) based on conditional values of the facts associated with all items in the record (or just some).
### File Correspondence
- Each fact will correspond to its own .csv file. Since many items can be associated with a single fact, and many facts can be associated with a single fact (and eventually, facts with themselves, and items with themselves), there must be one "master" node which corresponds to the proper item and fact.
- When a fact is inputted through dlog by a user, it is entered into the corresponding csv, no matter what item(s) were specified along with the fact input. Instead, the item master .csv file will also have a new line of input corresponding to the fact, such that an item .csv file will contain a fact column with multiple different fact keys, and a fact value column with a number of different fact values.
    - Ex. `dlog item health sleep 5hr` or `dlog sleep 5hr -i health` will write the record { "sleep",  "5", "hr", "<date>" } to {DATA_DIR}/fact/sleep.csv and to {DATA_DIR}/item/health.csv.
    - _Items_ will be tracked through a single item definition

## todo
- [ ] Make top level args (index 1 and 2) be free values corresponding to fact name and value, respectively
- [ ] Add unit of measure
- [ ] Figure out if you're going to implement the cmd flow through enums or if you should give up on that project and just make it all through functions or fieldless structs

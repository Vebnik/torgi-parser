## Cian api

#### URL - `/api/parse/cian/:pages/:region/:type/:object`
pages - page count for parse (28 elements per page)    <br>
region - [region](https://www.cian.ru/cian-api/site/v1/get-regions/) id (ex. 1 Moscow)  <br>
type - see type table    <br>
object - see type table   <br>

### Type table
```json
    "types": {
        "flatsale" | "flatrent":  {
            "name": "Комнаты",
            "objects": {
                    "комната": 0,
                    "1-комн. квартира": 1,
                    "2-комн. квартира": 2,
                    "3-комн. квартира": 3,
                    "4-комн. квартира": 4,
                    "5-комн. квартира": 5,
                    "многокомнатная квартира": 6,
                    "квартира свободной планировки": 7,
                    "студия": 9,
                    "койко-место": 10
            }
        },
        "suburbansale" | "suburbanrent": {
            "name": "Участки",
            "objects": {
                "дом": 1,
                "участок": 3,
                "таунхаус": 4
            }
        },
        "commercialsale" | "commercialrent": {
            "name": "Офисы",
            "objects": {
                "офис": 1,
                "торговая площадь": 2,
                "склад": 3,
                "помещение": 5,
                "гараж": 6
            }
        },
    }
```

### Example

`Студия в Москве` -> `/api/parse/cian/1/1/flatsale/9` <br>
`Гараж в Адыгее` -> `/api/parse/cian/1/4553/commercialrent/6` <br>

## Torgi api

#### URL - `/api/parse/torgi/:query/:pages`
query - query for search lot
pages - page count for parse (28 elements per page)    <br>

### Example
`Студия в Москве`   -> `/api/parse/torgi/студия в млскве/1` <br>
`Гараж в Адыгее`    -> `/api/parse/torgi/гараж в Адыгее/1` <br>
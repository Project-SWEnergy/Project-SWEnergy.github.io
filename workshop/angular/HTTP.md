# HTTP
## Installazione per test

Per i test installiamo un server locale che possa contenere JSON:

	npm install -g json-server

Creiamo un file contenente i dati in database:

	npm install touch-cli -g
	touch db.json

Inseriamo i dati nel file *db.json*:

	{
	    "locations": [
	        {
	            "id": 0,
	            "name": "Acme Fresh Start Housing",
	            "city": "Chicago",
	            "state": "IL",
	            "photo": "/assets/logo.jpg",
	            "availableUnits": 4,
	            "wifi": true,
	            "laundry": true
	          },
	          {
	            "id": 1,
	            "name": "A113 Transitional Housing",
	            "city": "Santa Monica",
	            "state": "CA",
	            "photo": "/assets/logo.jpg",
	            "availableUnits": 0,
	            "wifi": false,
	            "laundry": true
	          },
	          {
	            "id": 2,
	            "name": "Warm Beds Housing Support",
	            "city": "Juneau",
	            "state": "AK",
	            "photo": "/assets/logo.jpg",
	            "availableUnits": 1,
	            "wifi": false,
	            "laundry": false
	          },
	          {
	            "id": 3,
	            "name": "Homesteady Housing",
	            "city": "Chicago",
	            "state": "IL",
	            "photo": "/assets/logo.jpg",
	            "availableUnits": 1,
	            "wifi": true,
	            "laundry": false
	          },
	          {
	            "id": 4,
	            "name": "Happy Homes Group",
	            "city": "Gary",
	            "state": "IN",
	            "photo": "/assets/logo.jpg",
	            "availableUnits": 1,
	            "wifi": true,
	            "laundry": false
	          },
	          {
	            "id": 5,
	            "name": "Hopeful Apartment Group",
	            "city": "Oakland",
	            "state": "CA",
	            "photo": "/assets/logo.jpg",
	            "availableUnits": 2,
	            "wifi": true,
	            "laundry": true
	          }
	    ]
	}

Avviamo il server utilizzando i dati appena caricati:

	json-server --watch db.json


## Utilizzo

Si utilizza il servizio creato in [Services](workshop/angular/Services.html) e si crea la variabile contenente l'url su cui si effettueranno le richieste, se si sta usando quello locale creato in [[#Installazione per test]] allora la variabile sarà:

	url : string  = 'http://localhost:3000/locations'

Ora modifichiamo i metodi presenti nel servizio per effettuare delle chiamate e ottenere questi dati.

	async getAllHousingLocation(): Promise<HousingLocation[]> {
	    const data = await fetch(this.url);
	    return await data.json() ?? [];
	  }
	
	  async getHousingLocationById(id : Number) : Promise<HousingLocation | undefined> {
	    const data = await fetch(`${this.url}/${id}`);
	    return await data.json() ?? [];
	  }

Modifichiamo il costruttore di home.component:

	 constructor() {
	    this.housingService
		    .getAllHousingLocation()
		    .then((housingLocationList: HousingLocation[]) => {
			    this.housingLocationList = this.housingLocationList;
	    });
	  }

Modifichiamo il costruttore di details.component:

	constructor() {
    const housingLocationId = Number(this.route.snapshot.params["id"])
    this.housingService.getHousingLocationById(housingLocationId)
	    .then(housingLocation => {
	        this.housingLocation = housingLocation;
      });
	}

## Variabili template

Precedute dal simbolo # in HTML.

	<section>
    <form>
        <input type="text" placeholder="Filter by city" #filter>
        <button class="primary" type="button" (click)="filterResults(filter.value)"> Search </button>
    </form>
	</section>
	<section class="results">
	    <app-housing-location 
	        *ngFor="let housingLocation of filteredLocationList"
	        [housingLocation] = "housingLocation" >
	    </app-housing-location>
	</section>


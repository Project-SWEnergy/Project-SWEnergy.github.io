# Axios

## Installazione
Da riga di comando:

	npm install axios


## Configurazione
Si genera un file "axios-config.ts" con il seguente contenuto:

	import axios from 'axios';
	
	const instance = axios.create({
		// Sostituisci con la tua URL di base
		baseURL: 'https://api.example.com',  

		// Imposta il timeout secondo le tue esigenze
		timeout: 5000,  
	});
	
	export default instance;


## Utilizzo
Viene utilizzato all'interno dei servizi come segue:

	import { Injectable } from '@angular/core';
	import axios from './axios-config';  // Importa la configurazione Axios
	
	@Injectable({
		ProvidedIn: 'root',
	})
	export class MyApiService {
		getData() {
		    return axios.get('/data');  // Esempio di chiamata GET
		 }

		postData(data: any) {
			return axios.post('/data', data);  // Esempio di chiamata POST
		 }
	}

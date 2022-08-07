<script type="typescript">
	import moment from 'moment';
	import axios from 'axios';

	let name = '';
	let date = moment().startOf('hour').add(1, 'hour').format('YYYY-MM-DDTHH:mm');

	let createEvent = async () => {
		console.log(import.meta.env);
		try {
			const resp = await axios({
				method: 'POST',
				url: `${import.meta.env.VITE_BACKEND_URL}/event`,
				data: {
					name,
					date: moment(date).toISOString()
				}
			});
			window.location.href = `/id/${resp.data}`;
		} catch (err) {
			console.error(err);
		}
	};
</script>

<main>
	<h1>Create Countdown</h1>

	<input style="grid-area: event-name" placeholder="Event Name" bind:value={name} />
	<input style="grid-area: event-date" type="datetime-local" bind:value={date} />

	<button style="grid-area: create-button" type="button" on:click={createEvent}>Create</button>
</main>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@100;200;300;400;500;600;700');

	main * {
		font-family: 'Roboto Mono', monospace;
	}

	main {
		display: grid;
		justify-content: center;
		height: 100vh;
		grid-template-areas:
			'.'
			'page-title'
			'event-name'
			'event-date'
			'create-button'
			'.';
		grid-template-rows: 1fr auto 3rem 3rem 3rem 1fr;
		gap: 1rem;
		background-color: rgb(33, 33, 33);
		color: rgb(224, 224, 224);
	}

	h1 {
		grid-area: page-title;
		margin: 0;
		text-align: center;
	}

	input,
	button {
		border-radius: 0.5rem;
		color: rgb(224, 224, 224);
		border: none;
	}

	input {
		background-color: rgb(55, 71, 79);
		padding: 1rem 1.5rem 1rem 1.5rem;
	}
	input::placeholder {
		color: rgb(224, 224, 224);
	}

	input:focus,
	input:focus-visible {
		border: none;
		outline: 2px solid rgb(25, 118, 210);
	}

	button {
		background-color: rgb(25, 118, 210);
		font-weight: bold;
	}
	button:hover {
		background-color: rgb(13, 71, 161);
		cursor: pointer;
	}
</style>

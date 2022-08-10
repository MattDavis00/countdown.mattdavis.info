<script type="text/typescript">
	import type { Countdown } from './[id]';
	import moment from 'moment';

	// Populated with data from the endpoint
	export let countdown: Countdown;
	let date = moment(countdown.date);
	let timeRemaining = moment.duration(date.diff(moment()));

	const updateTimer = async () => {
		date = moment(countdown.date);
		timeRemaining = moment.duration(date.diff(moment()));
	};

	// Could be done more effeciently than polling
	setInterval(updateTimer, 50);

	let shareButtonText = 'Share';

	const share = async () => {
		try {
			await navigator.clipboard.writeText(window.location.toString());
			shareButtonText = 'Copied ✅';
			setTimeout(() => (shareButtonText = 'Share'), 2000);
		} catch (err) {
			console.error(err);
		}
	};
</script>

<html lang="en" data-theme="dark">
	<head>
		<title>{countdown.name} - Countdown Timer</title>
	</head>

	<main class="grid-layout">
		<h1 style="grid-area: event-name">{countdown.name}</h1>
		<h2 style="grid-area: event-date">{date.format('LLLL')}</h2>

		<h3 class="time-remaining">
			<span>{Math.floor(timeRemaining.asDays())} days</span>
			<span>{timeRemaining.hours()} hours</span>
			<span>{timeRemaining.minutes()} minutes</span>
			<span>{timeRemaining.seconds()} seconds</span>
		</h3>

		<div class="buttons">
			<button class="button share-button" type="button" on:click={share}>{shareButtonText}</button>
			<a class="button new-event-button" href="/" type="button">Create new event</a>
		</div>
	</main>
</html>

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
			'event-name'
			'event-date'
			'time-remaining'
			'buttons'
			'.';
		grid-template-rows: 1fr auto auto auto 3rem 3rem 1fr;
		gap: 3rem;
		background-color: rgb(33, 33, 33);
		color: rgb(224, 224, 224);
		justify-items: center;
		padding: 2rem;
		box-sizing: border-box;
	}

	.time-remaining {
		grid-area: time-remaining;
		display: flex;
		flex-direction: column;
		text-align: center;
		gap: 10px;
	}

	h1 {
		grid-area: event-name;
		margin: 0;
	}

	h2 {
		grid-area: event-date;
	}

	.buttons {
		grid-area: buttons;
		display: grid;
		grid-template-rows: 3rem 3rem;
		grid-template-columns: 100%;
		gap: 1rem;
		width: 100%;
		max-width: 400px;
		justify-content: center;
	}

	.button {
		border-radius: 0.5rem;
		color: rgb(224, 224, 224);
		border: none;
		background-color: rgb(25, 118, 210);
		font-weight: bold;
		text-decoration: none;
		align-items: center;
		justify-content: center;
		font-size: 0.9rem;
		display: inline-flex;
	}

	.button:hover {
		background-color: rgb(13, 71, 161);
		cursor: pointer;
	}

	h1,
	h2,
	h3 {
		margin: 0;
		text-align: center;
	}
</style>

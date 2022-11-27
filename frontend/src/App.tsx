import React, { useEffect, useState } from 'react';
import './App.css';

function App() {
	const [num, setNum] = useState(5);

	useEffect(() => {
		console.log('use effect');

		const getData = async () => {
			const num = await (await fetch('http://localhost:8080/json')).json();
			console.log(num);
			setNum(num);
		};

		getData();
	}, []);
	return (
		<div className='App'>
			<h2>Hello, world!!!</h2>
			<h3>{num}</h3>
		</div>
	);
}

export default App;

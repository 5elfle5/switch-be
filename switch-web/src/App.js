import React, { useState, useEffect } from 'react';
import './App.css';

function App() {
  const [data, setData] = useState([]);

  useEffect(() => {
    fetch("http://localhost:8080/speak", { method: 'POST', body: 'will you be my friend?'})
      .then((response) => response.json())
      .then(data => setData(data.text));
  });

  return (
    <div>
      <ul>
        {data}
      </ul>
    </div>
  );
}

export default App;

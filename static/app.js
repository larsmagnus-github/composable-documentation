document.getElementById('url-form').addEventListener('submit', async function(event) {
  event.preventDefault();
  const urlInput = document.getElementById('url-input').value;

  // Send POST request to the backend
  const response = await fetch('/convert', {
      method: 'POST',
      headers: {
          'Content-Type': 'application/json',
      },
      body: JSON.stringify({ url: urlInput }),
  });

  if (response.ok) {
      const data = await response.json();
      document.getElementById('markdown-output').value = data.markdown;
  } else {
      const errorText = await response.text();
      alert('Error: ' + errorText);
  }
});
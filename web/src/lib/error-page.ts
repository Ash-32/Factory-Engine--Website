export function renderErrorPage(): string {
  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Something went wrong</title>
  <style>
    body {
      margin: 0;
      min-height: 100vh;
      display: flex;
      align-items: center;
      justify-content: center;
      background: #f3ede2;
      color: #1a1a1a;
      font-family: ui-sans-serif, system-ui, sans-serif;
      padding: 1.5rem;
    }
    main { max-width: 28rem; text-align: center; }
    h1 { font-size: 1.25rem; font-weight: 600; margin: 0 0 0.5rem; }
    p { font-size: 0.875rem; color: #6b665d; line-height: 1.6; margin: 0 0 1.5rem; }
    a {
      color: #1a1a1a;
      font-size: 0.875rem;
      font-weight: 500;
      text-decoration: underline;
      text-underline-offset: 3px;
    }
  </style>
</head>
<body>
  <main>
    <h1>This page didn't load</h1>
    <p>Something went wrong on our end. Try refreshing, or return to the home page.</p>
    <a href="/">Go home</a>
  </main>
</body>
</html>`;
}

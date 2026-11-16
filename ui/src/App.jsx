import SeamCarve from './SeamCarve.jsx'

function App() {
  return <>
    <header className="container" style={{ display: 'flex',  alignItems: 'center', justifyContent: 'space-between' }}>
      <h1 style={{ margin: '0' }}>Seam Carving</h1>
      <nav>
        <ul>
          <li><a>Seam Carving</a></li>
          <li><a>Object Removal</a></li>
        </ul>
      </nav>
    </header>
    <main className="container">
      <SeamCarve />
    </main>
  </>
}

export default App

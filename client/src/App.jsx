import { useState } from 'react'
import './App.css'

function App() {
  const [serverUrl, setServerUrl] = useState('http://we.makegumi.jp')
  const [message, setMessage] = useState('Hello from React!')
  const [response, setResponse] = useState('')
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState('')

  const decodeServerResponse = (responseText) => {
    // "echo: message=Hello%20from%20React!" -> "Hello from React!"
    if (responseText.startsWith('echo: message=')) {
      const encodedMessage = responseText.replace('echo: message=', '')
      try {
        const decodedMessage = decodeURIComponent(encodedMessage)
        return `echo: ${decodedMessage}`
      } catch (err) {
        // デコードに失敗した場合は元のテキストを返す
        return responseText
      }
    }
    // "echo: "で始まらない場合はそのまま返す
    return responseText
  }

  const sendPostRequest = async () => {
    if (!message.trim()) {
      setError('メッセージを入力してください')
      return
    }

    setLoading(true)
    setError('')
    setResponse('')

    try {
      const res = await fetch(serverUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'text/plain',
        },
        body: message
      })

      if (!res.ok) {
        throw new Error(`HTTP ${res.status}: ${res.statusText}`)
      }

      const responseText = await res.text()
      setResponse(responseText)
    } catch (err) {
      setError(`エラー: ${err.message}`)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="container">
      <h1>🚀 WASM HTTP Server Client</h1>
      <p>WebAssemblyサーバーにPOSTリクエストを送信するテストクライアントです</p>
      
      <div className="form-group">
        <label htmlFor="serverUrl">サーバーURL:</label>
        <input
          id="serverUrl"
          type="text"
          value={serverUrl}
          onChange={(e) => setServerUrl(e.target.value)}
          placeholder="http://we.makegumi.jp"
        />
      </div>

      <div className="form-group">
        <label htmlFor="message">メッセージ:</label>
        <textarea
          id="message"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          placeholder="送信するメッセージを入力してください"
          rows={4}
        />
      </div>

      <button 
        onClick={sendPostRequest} 
        disabled={loading}
        className="send-button"
      >
        {loading ? '送信中...' : 'POST送信 📤'}
      </button>

      {error && (
        <div className="error">
          ❌ {error}
        </div>
      )}

      {response && (
        <div className="response">
          <h3>🎉 レスポンス:</h3>
          <pre>{response}</pre>
        </div>
      )}
    </div>
  )
}

export default App

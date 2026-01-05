import { NextResponse } from 'next/server'

export const runtime = 'nodejs'

type Params = { path?: string[] }

async function proxy(req: Request, { params }: { params: Params }) {
  const path = (params.path || []).join('/')
  const targetUrl = `http://localhost:8000/api/${path}`

  const controller = new AbortController()
  const timeoutId = setTimeout(() => controller.abort(), 120000)

  try {
    const reqHeaders = new Headers(req.headers)
    reqHeaders.delete('host')
    reqHeaders.delete('connection')
    reqHeaders.delete('content-length')

    const upstreamResp = await fetch(targetUrl, {
      method: req.method,
      headers: reqHeaders,
      body: req.method === 'GET' || req.method === 'HEAD' ? undefined : await req.arrayBuffer(),
      signal: controller.signal,
      redirect: 'manual',
    })

    const respHeaders = new Headers(upstreamResp.headers)
    respHeaders.delete('content-encoding')
    respHeaders.delete('transfer-encoding')
    respHeaders.delete('connection')

    const buf = await upstreamResp.arrayBuffer()

    return new NextResponse(buf, {
      status: upstreamResp.status,
      headers: respHeaders,
    })
  } catch (e: any) {
    if (e?.name === 'AbortError') {
      return NextResponse.json({ success: false, data: 'Upstream request timeout' }, { status: 504 })
    }
    return NextResponse.json({ success: false, data: e?.message || 'Proxy error' }, { status: 502 })
  } finally {
    clearTimeout(timeoutId)
  }
}

export { proxy as GET, proxy as POST, proxy as PUT, proxy as PATCH, proxy as DELETE }

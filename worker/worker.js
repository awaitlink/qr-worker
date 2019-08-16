addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request));
});

async function handleRequest(request) {
    const { handle_request } = wasm_bindgen;
    await wasm_bindgen(wasm);
    const output = handle_request(request.url);

    let res = new Response(output, { status: 200 });
    res.headers.set('Content-type', 'image/svg+xml');
    return res;
}

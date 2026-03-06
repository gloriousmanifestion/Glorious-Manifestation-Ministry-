/**
 * Safely parse JSON from a fetch Response.
 * Handles empty bodies and invalid JSON to avoid "Unexpected end of JSON input" errors.
 */
export async function safeJson<T = unknown>(res: Response): Promise<T> {
	const text = await res.text();
	if (!text || !text.trim()) {
		return {} as T;
	}
	try {
		return JSON.parse(text) as T;
	} catch {
		throw new Error('Invalid response from server');
	}
}

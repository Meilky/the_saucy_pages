export type Handler = () => void;

export class Router {
	private routes: Map<RegExp, Handler>;

	public constructor() {
		this.routes = new Map();
	}

	public addRoute(match: RegExp, handler: Handler): void {
		this.routes.set(match, handler);
	}

	public removeRoute(match: RegExp): void {
		this.routes.delete(match);
	}

	public getHandlerByMatch(against: string): Handler | undefined {
		for (const [match, handler] of this.routes.entries()) {
			if (match.test(against)) {
				return handler;
			}
		}
	}
}

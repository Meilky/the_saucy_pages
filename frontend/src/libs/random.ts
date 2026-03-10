export function getRandomArbitrary(min: number, max: number): number {
	if (min >= max) {
		throw new Error("min must be smaller than max!");
	}

	return Math.random() * (max - min) + min;
}

export function getRandomInt(min: number, max: number, exclude: number[] = []): number {
	const num = Math.floor(getRandomArbitrary(min, max));

	return num in exclude ? getRandomInt(min, max) : num;
}

export function getRandomDistinctIntArray(
	count: number,
	min: number,
	max: number,
	exclude: number[] = [],
): number[] {
	if (count > max) {
		throw new Error("count must be smaller or equal to max");
	}

	if (min >= max) {
		throw new Error("min must be smaller than max!");
	}

	const excludeClone: number[] = [...exclude];
	const picks: number[] = [];

	while (count > 0) {
		const num = getRandomInt(min, max, excludeClone);

		picks.push(num);
		exclude.push(num);

		count--;
	}

	return picks;
}

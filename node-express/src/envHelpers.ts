export function setDefaultEnvVar(
  key: string,
  defaultValue: string | undefined
) {
  const value = process.env[key];
  if (value === undefined) {
    process.env[key] = defaultValue;
  }
}

export function getRequiredEnvVar(key: string): string {
  const value = process.env[key];
  if (value === undefined) {
    throw Error(`${key} env var does not exist!`);
  }
  return value;
}

export function update_progress(percentage) {
  self.postMessage({ type: "update-percentage", message: percentage });
}

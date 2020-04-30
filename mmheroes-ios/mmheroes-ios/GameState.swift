struct GameState: Codable {
    var seed: UInt64
    var input: [MMHEROES_Input]
}

extension GameState {
    init() {
        seed = .random(in: 0 ... .max)
        input = []
    }
}

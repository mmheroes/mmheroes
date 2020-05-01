import Foundation

let nscoderJSONEncoder = JSONEncoder()
let nscoderJSONDecoder = JSONDecoder()

extension NSCoder {
    func encodeEncodable<T: Encodable>(_ encodable: T, forKey key: String) throws {
        let data = try nscoderJSONEncoder.encode(encodable)
        encode(data as Any, forKey: key)
        if #available(iOS 9.0, *), let error = self.error {
            throw error
        }
    }

    func decodeDecodable<T: Decodable>(_ type: T.Type, forKey key: String) throws -> T {
        guard let object = decodeObject(forKey: key) else {
            throw DecodingError
                .valueNotFound(type, .init(codingPath: [], debugDescription: ""))
        }
        guard let data = object as? Data else {
            throw DecodingError
                .typeMismatch(Data.self, .init(codingPath: [], debugDescription: ""))
        }
        if #available(iOS 9.0, *), let error = self.error {
            throw error
        }
        return try nscoderJSONDecoder.decode(type, from: data)
    }
}

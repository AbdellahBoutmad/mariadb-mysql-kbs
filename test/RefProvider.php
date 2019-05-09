<?php
declare(strict_types = 1);
namespace Williamdes\MariaDBMySQLKBS\Test;

use \Swaggest\JsonSchema\RemoteRefProvider;
use \stdClass;

class RefProvider implements RemoteRefProvider {

    /**
     * Preloaded urn schemas
     *
     * @var stdClass[]
     */
    private $urnSchemas = [];

    public function __construct()
    {
        $files = glob(__DIR__ . "/../schemas/*.json");
        if($files === false) {
            return;
        } else {
            foreach ($files as $filename) {
                $schema = json_decode((string) file_get_contents($filename));
                if(isset($schema) && isset($schema->{'$id'}) && ! is_null($schema)) {
                    $this->urnSchemas[$schema->{'$id'}] = $schema;
                }
            }
        }
    }

    /**
     * @param string $url The file url
     * @return \stdClass|false json_decode of $url resource content
     */
    public function getSchemaData($url)
    {
        if(isset($this->urnSchemas[$url])) {// Handle urn: urls
            return $this->urnSchemas[$url];
        } else if(is_file($url)) {// Handle file
            return json_decode((string) file_get_contents($url));
        } else {// Handle URL
            return json_decode((string) file_get_contents(rawurldecode($url)));
        }
    }
}

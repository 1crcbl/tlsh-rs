import boto3

# https://registry.opendata.aws/1000-genomes/
# https://docs.opendata.aws/1000genomes/readme.html

s3 = boto3.client('s3', region_name='us-east-1')
objects = s3.list_objects(Bucket='1000genomes')
print(objects)

# s3.download_file('1000genomes', '1000G_2504_high_coverage/additional_698_related/data/ERR3988763/HG00418.final.cram', 'cram.cram')
